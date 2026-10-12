//! INSERT, UPDATE, DELETE and MERGE.

use crate::Row;
use crate::ast::{self, Join, Node, Rel, Statement};
use crate::expr::{Assignable, Assignments, Base, Operand};
use crate::query::{FromItem, Projection, Rows, Select, Source, Sources};
use crate::schema::{Alias, Constraint, Scoped, Table};
use crate::types::*;
use std::marker::PhantomData;

/// A required column without a value yet.
pub struct Missing;

/// A required column with a value.
pub struct Set;

/// The values of an inserted row; the generated builders wrap it and track
/// which required columns are set.
#[derive(Default)]
pub struct InsertRow {
    values: Vec<(&'static str, Node)>,
    overriding: bool,
}

impl InsertRow {
    /// Sets `column`, of SQL type `C`.
    pub fn set<C: PgType, V: ColumnValue<C>>(
        mut self,
        column: &'static str,
        value: V,
    ) -> InsertRow {
        self.values.push((column, value.into_value()));
        self
    }

    /// Sets an identity column GENERATED ALWAYS, with OVERRIDING SYSTEM VALUE.
    pub fn set_identity<C: PgType, V: ColumnValue<C>>(
        mut self,
        column: &'static str,
        value: V,
    ) -> InsertRow {
        self.overriding = true;
        self.set::<C, V>(column, value)
    }
}

/// A row builder with every required column set.
pub trait Insertable {
    type Table: Table;
    #[doc(hidden)]
    fn into_row(self) -> InsertRow;
}

fn target<T: Table>(table: T) -> (ast::From, T::Source) {
    table.into_from()
}

fn returning<P: Projection>(projection: P) -> Vec<ast::Item> {
    let mut items = Vec::new();
    projection.items(&mut items);
    items
}

/// An INSERT statement into `T`, returning rows `R`.
pub struct Insert<T: Table, R = ()> {
    source: T::Source,
    ast: ast::Insert,
    row: PhantomData<fn() -> R>,
}

impl<T: Table> Insert<T> {
    pub(crate) fn new<I: Insertable<Table = T>>(table: T, row: I) -> Insert<T> {
        let (from, source) = target(table);
        let mut insert = Insert {
            source,
            ast: ast::Insert {
                target: from,
                columns: Vec::new(),
                overriding: false,
                rows: Vec::new(),
                query: None,
                conflict: None,
                returning: Vec::new(),
            },
            row: PhantomData,
        };
        insert.push(row.into_row());
        insert
    }

    /// `INSERT ... DEFAULT VALUES`.
    pub(crate) fn defaults(table: T) -> Insert<T> {
        let (from, source) = target(table);
        Insert {
            source,
            ast: ast::Insert {
                target: from,
                columns: Vec::new(),
                overriding: false,
                rows: Vec::new(),
                query: None,
                conflict: None,
                returning: Vec::new(),
            },
            row: PhantomData,
        }
    }

    /// `INSERT ... SELECT`: the values of each row are built from the sources
    /// of `query`.
    pub(crate) fn select<S: Sources, I: Insertable<Table = T>>(
        table: T,
        query: Select<S>,
        row: impl FnOnce(T::Insert, S::View) -> I,
    ) -> Insert<T> {
        let mut insert = Insert::defaults(table);
        let (scope, mut select) = query.into_parts();
        let row = row(T::Insert::default(), scope.view()).into_row();
        insert.ast.overriding = row.overriding;
        for (column, node) in row.values {
            insert.ast.columns.push(column);
            select.items.push(ast::Item { node, alias: None });
        }
        insert.ast.query = Some(Statement::Select(Box::new(select)));
        insert
    }

    fn push(&mut self, row: InsertRow) {
        self.ast.overriding |= row.overriding;
        let rows = &mut self.ast.rows;
        let columns = &mut self.ast.columns;
        let mut values = vec![Node::Default; columns.len()];
        for (column, node) in row.values {
            match columns.iter().position(|c| *c == column) {
                Some(index) => values[index] = node,
                None => {
                    columns.push(column);
                    rows.iter_mut().for_each(|r| r.push(Node::Default));
                    values.push(node);
                }
            }
        }
        rows.push(values);
    }

    /// Another row of a multi-row VALUES list; columns set in only some rows
    /// get DEFAULT in the others.
    pub fn values<I: Insertable<Table = T>>(mut self, row: impl FnOnce(T::Insert) -> I) -> Self {
        self.push(row(T::Insert::default()).into_row());
        self
    }

    /// `ON CONFLICT (columns)`, the columns of a unique index.
    pub fn on_conflict<E: crate::query::Exprs>(
        self,
        columns: impl FnOnce(T::Source) -> E,
    ) -> Conflict<T> {
        let columns = columns(self.source.clone()).into_nodes();
        Conflict {
            insert: self,
            target: Some(ast::Conflict::Columns(columns, None)),
        }
    }

    /// `ON CONFLICT (columns) WHERE predicate`, for a partial unique index.
    pub fn on_conflict_where<E: crate::query::Exprs, P: Operand>(
        self,
        columns: impl FnOnce(T::Source) -> E,
        predicate: impl FnOnce(T::Source) -> P,
    ) -> Conflict<T>
    where
        Base<P>: Boolean,
    {
        let columns = columns(self.source.clone()).into_nodes();
        let predicate = predicate(self.source.clone()).into_node();
        Conflict {
            insert: self,
            target: Some(ast::Conflict::Columns(columns, Some(predicate))),
        }
    }

    /// `ON CONFLICT ON CONSTRAINT name`.
    pub fn on_constraint(self, constraint: Constraint<T>) -> Conflict<T> {
        Conflict {
            insert: self,
            target: Some(ast::Conflict::Constraint(constraint.name)),
        }
    }

    /// `RETURNING`, over the inserted row.
    pub fn returning<P: Projection>(
        self,
        projection: impl FnOnce(T::Source) -> P,
    ) -> Insert<T, P::Row> {
        let items = returning(projection(self.source.clone()));
        self.with_returning(items)
    }

    /// `RETURNING` over the `old` and `new` rows; `old` is NULL unless the
    /// row was updated by ON CONFLICT DO UPDATE.
    pub fn returning_old_new<P: Projection>(
        self,
        projection: impl FnOnce((<T::Source as Source>::Nullable, T::Source)) -> P,
    ) -> Insert<T, P::Row>
    where
        <T::Source as Source>::Nullable: Scoped,
    {
        let old = <<T::Source as Source>::Nullable as Scoped>::scoped(Alias(Rel::Old));
        let new = T::Source::scoped(Alias(Rel::New));
        let items = returning(projection((old, new)));
        self.with_returning(items)
    }

    fn with_returning<R>(mut self, items: Vec<ast::Item>) -> Insert<T, R> {
        self.ast.returning = items;
        Insert {
            source: self.source,
            ast: self.ast,
            row: PhantomData,
        }
    }
}

/// An INSERT with an ON CONFLICT target, waiting for its action.
pub struct Conflict<T: Table> {
    insert: Insert<T>,
    target: Option<ast::Conflict>,
}

impl<T: Table> Conflict<T> {
    pub fn do_nothing(mut self) -> Insert<T> {
        self.insert.ast.conflict = Some(ast::OnConflict {
            target: self.target,
            update: None,
        });
        self.insert
    }

    /// `DO UPDATE SET`, from the existing row and the `excluded` row.
    pub fn do_update<A: Assignments>(
        self,
        set: impl FnOnce((T::Source, T::Source)) -> A,
    ) -> Insert<T> {
        let set = set(self.rows());
        self.update(set, None)
    }

    /// `DO UPDATE SET ... WHERE condition`.
    pub fn do_update_where<A: Assignments, E: Operand>(
        self,
        set: impl FnOnce((T::Source, T::Source)) -> A,
        condition: impl FnOnce((T::Source, T::Source)) -> E,
    ) -> Insert<T>
    where
        Base<E>: Boolean,
    {
        let set = set(self.rows());
        let condition = condition(self.rows()).into_node();
        self.update(set, Some(condition))
    }

    /// The existing row and the `excluded` row proposed for insertion.
    fn rows(&self) -> (T::Source, T::Source) {
        let excluded = T::Source::scoped(Alias(Rel::Excluded));
        (self.insert.source.clone(), excluded)
    }

    fn update<A: Assignments>(mut self, set: A, condition: Option<Node>) -> Insert<T> {
        let set = assignments(set, self.insert.ast.target.rel);
        self.insert.ast.conflict = Some(ast::OnConflict {
            target: self.target,
            update: Some((set, condition)),
        });
        self.insert
    }
}

fn assignments<A: Assignments>(set: A, target: u64) -> Vec<ast::Assignment> {
    set.into_assignments()
        .into_iter()
        .map(|a| {
            assert!(
                a.rel == Rel::Source(target),
                "SET assigns columns of the target table"
            );
            ast::Assignment {
                columns: a.columns,
                value: a.value,
            }
        })
        .collect()
}

impl<T: Table, R: Row> Rows for Insert<T, R> {
    type Row = R;
    fn into_statement(self) -> Statement {
        Statement::Insert(Box::new(self.ast))
    }
}

/// The scope of an UPDATE or DELETE: a table first, then the tables of the
/// FROM or USING list, all joined with inner joins.
pub trait Target: Sources {
    type Table: Scoped;
    fn table(&self) -> Self::Table;
}

impl<T: Scoped> Target for (T,) {
    type Table = T;
    fn table(&self) -> T {
        self.0.clone()
    }
}

macro_rules! targets {
    ($($s:ident $i:tt),*) => {
        impl<T: Scoped, $($s: Source),*> Target for (T, $($s,)*)
        where
            (T, $($s,)*): Sources,
        {
            type Table = T;
            fn table(&self) -> T {
                self.0.clone()
            }
        }
    };
}

targets!(S1 1);
targets!(S1 1, S2 2);
targets!(S1 1, S2 2, S3 3);
targets!(S1 1, S2 2, S3 3, S4 4);

/// The parts of a SELECT that an UPDATE or DELETE takes over.
fn split(select: ast::Select) -> (ast::From, Vec<Join>, Option<Node>) {
    assert!(
        select.from[1..]
            .iter()
            .all(|j| matches!(j.kind, "JOIN" | "CROSS JOIN")),
        "UPDATE and DELETE join other tables with inner joins only"
    );
    let mut joins = select.from.into_iter();
    let target = joins.next().expect("a target table").from;
    let mut filter = None;
    let mut others = Vec::new();
    for mut join in joins {
        if let Some(on) = join.on.take() {
            filter = Some(match filter {
                Some(previous) => Node::binary("AND", previous, on),
                None => on,
            });
        }
        others.push(join);
    }
    let filter = match (filter, select.filter) {
        (Some(on), Some(filter)) => Some(Node::binary("AND", on, filter)),
        (on, filter) => on.or(filter),
    };
    (target, others, filter)
}

impl<S: Target> Select<S> {
    /// `UPDATE` the first table of the query, with the other tables in FROM
    /// and the filters and join conditions in WHERE.
    pub fn update<A: Assignments>(self, set: impl FnOnce(S::View) -> A) -> Update<S> {
        let (scope, select) = self.into_parts();
        let set = set(scope.view());
        let (target, from, filter) = split(select);
        let set = assignments(set, target.rel);
        Update {
            scope,
            ast: ast::Update {
                target,
                set,
                from,
                filter,
                returning: Vec::new(),
            },
            row: PhantomData,
        }
    }

    /// `DELETE FROM` the first table of the query, with the other tables in
    /// USING.
    pub fn delete(self) -> Delete<S> {
        let (scope, select) = self.into_parts();
        let (target, using, filter) = split(select);
        Delete {
            scope,
            ast: ast::Delete {
                target,
                using,
                filter,
                returning: Vec::new(),
            },
            row: PhantomData,
        }
    }
}

/// An UPDATE statement.
pub struct Update<S, R = ()> {
    scope: S,
    ast: ast::Update,
    row: PhantomData<fn() -> R>,
}

impl<S: Target> Update<S> {
    pub fn returning<P: Projection>(
        mut self,
        projection: impl FnOnce(S::View) -> P,
    ) -> Update<S, P::Row> {
        self.ast.returning = returning(projection(self.scope.view()));
        Update {
            scope: self.scope,
            ast: self.ast,
            row: PhantomData,
        }
    }

    /// `RETURNING` over the `old` and `new` versions of the updated row.
    pub fn returning_old_new<P: Projection>(
        mut self,
        projection: impl FnOnce((S::Table, S::Table)) -> P,
    ) -> Update<S, P::Row> {
        let old = S::Table::scoped(Alias(Rel::Old));
        let new = S::Table::scoped(Alias(Rel::New));
        self.ast.returning = returning(projection((old, new)));
        Update {
            scope: self.scope,
            ast: self.ast,
            row: PhantomData,
        }
    }
}

impl<S, R: Row> Rows for Update<S, R> {
    type Row = R;
    fn into_statement(self) -> Statement {
        Statement::Update(Box::new(self.ast))
    }
}

/// A DELETE statement.
pub struct Delete<S, R = ()> {
    scope: S,
    ast: ast::Delete,
    row: PhantomData<fn() -> R>,
}

impl<S: Target> Delete<S> {
    pub fn returning<P: Projection>(
        mut self,
        projection: impl FnOnce(S::View) -> P,
    ) -> Delete<S, P::Row> {
        self.ast.returning = returning(projection(self.scope.view()));
        Delete {
            scope: self.scope,
            ast: self.ast,
            row: PhantomData,
        }
    }

    /// `RETURNING` over the `old` row and the `new` row, which is NULL.
    pub fn returning_old_new<P: Projection>(
        mut self,
        projection: impl FnOnce((S::Table, <S::Table as Source>::Nullable)) -> P,
    ) -> Delete<S, P::Row>
    where
        <S::Table as Source>::Nullable: Scoped,
    {
        let old = S::Table::scoped(Alias(Rel::Old));
        let new = <<S::Table as Source>::Nullable as Scoped>::scoped(Alias(Rel::New));
        self.ast.returning = returning(projection((old, new)));
        Delete {
            scope: self.scope,
            ast: self.ast,
            row: PhantomData,
        }
    }
}

impl<S, R: Row> Rows for Delete<S, R> {
    type Row = R;
    fn into_statement(self) -> Statement {
        Statement::Delete(Box::new(self.ast))
    }
}

/// Whether a MERGE has a clause of some kind, at the type level.
pub trait Flag {
    type Null: Nullity;
}

pub struct No;
pub struct Yes;

impl Flag for No {
    type Null = NotNull;
}

impl Flag for Yes {
    type Null = IsNull;
}

/// A MERGE statement into `T` from the source `U`; `I` and `D` record
/// whether it has INSERT and DELETE actions, which make the `old` and `new`
/// rows of RETURNING nullable.
pub struct Merge<T: Table, U, I = No, D = No, R = ()> {
    target: T::Source,
    source: U,
    ast: ast::Merge,
    row: PhantomData<fn() -> (I, D, R)>,
}

/// `MERGE INTO table USING source ON condition`.
pub(crate) fn merge<T: Table, F: FromItem, E: Operand>(
    table: T,
    source: F,
    on: impl FnOnce((T::Source, F::Source)) -> E,
) -> Merge<T, F::Source>
where
    Base<E>: Boolean,
{
    let (target_from, target) = target(table);
    let (source_from, source) = source.into_from();
    let on = on((target.clone(), source.clone())).into_node();
    Merge {
        target,
        source,
        ast: ast::Merge {
            target: target_from,
            source: source_from,
            on,
            clauses: Vec::new(),
            returning: Vec::new(),
        },
        row: PhantomData,
    }
}

impl<T: Table, U: Source, I: Flag, D: Flag> Merge<T, U, I, D> {
    fn when<I2, D2>(mut self, kind: &'static str, action: ast::Action) -> Merge<T, U, I2, D2> {
        self.ast.clauses.push(ast::When {
            kind,
            condition: None,
            action,
        });
        Merge {
            target: self.target,
            source: self.source,
            ast: self.ast,
            row: PhantomData,
        }
    }

    fn scope(&self) -> (T::Source, U) {
        (self.target.clone(), self.source.clone())
    }

    /// `WHEN MATCHED THEN UPDATE SET`.
    pub fn when_matched_update<A: Assignments>(
        self,
        set: impl FnOnce((T::Source, U)) -> A,
    ) -> Self {
        let set = assignments(set(self.scope()), self.ast.target.rel);
        self.when("MATCHED", ast::Action::Update(set))
    }

    /// `WHEN MATCHED THEN DELETE`.
    pub fn when_matched_delete(self) -> Merge<T, U, I, Yes> {
        self.when("MATCHED", ast::Action::Delete)
    }

    /// `WHEN MATCHED THEN DO NOTHING`.
    pub fn when_matched_do_nothing(self) -> Self {
        self.when("MATCHED", ast::Action::Nothing)
    }

    /// `WHEN NOT MATCHED THEN INSERT`, the row built from the source.
    pub fn when_not_matched_insert<V: Insertable<Table = T>>(
        self,
        row: impl FnOnce(T::Insert, U) -> V,
    ) -> Merge<T, U, Yes, D> {
        let row = row(T::Insert::default(), self.source.clone()).into_row();
        let (columns, values) = row.values.into_iter().unzip();
        self.when("NOT MATCHED", ast::Action::Insert(columns, values))
    }

    /// `WHEN NOT MATCHED BY SOURCE THEN DELETE`.
    pub fn when_not_matched_by_source_delete(self) -> Merge<T, U, I, Yes> {
        self.when("NOT MATCHED BY SOURCE", ast::Action::Delete)
    }

    /// `RETURNING` over the target and source rows.
    pub fn returning<P: Projection>(
        mut self,
        projection: impl FnOnce((T::Source, U)) -> P,
    ) -> Merge<T, U, I, D, P::Row> {
        self.ast.returning = returning(projection(self.scope()));
        Merge {
            target: self.target,
            source: self.source,
            ast: self.ast,
            row: PhantomData,
        }
    }

    /// `RETURNING` over the `old` and `new` rows, nullable when the MERGE
    /// inserts or deletes rows, and the source row.
    pub fn returning_old_new<P: Projection>(
        mut self,
        projection: impl FnOnce((MaybeNull<T::Source, I>, MaybeNull<T::Source, D>, U)) -> P,
    ) -> Merge<T, U, I, D, P::Row>
    where
        I: Maybe<T::Source>,
        D: Maybe<T::Source>,
    {
        let old = I::scoped(Alias(Rel::Old));
        let new = D::scoped(Alias(Rel::New));
        self.ast.returning = returning(projection((old, new, self.source.clone())));
        Merge {
            target: self.target,
            source: self.source,
            ast: self.ast,
            row: PhantomData,
        }
    }
}

/// The source `S`, nullable when the flag is `Yes`.
pub trait Maybe<S> {
    type Source: Scoped;
    fn scoped(alias: Alias) -> Self::Source;
}

impl<S: Scoped> Maybe<S> for No {
    type Source = S;
    fn scoped(alias: Alias) -> S {
        S::scoped(alias)
    }
}

impl<S: Scoped> Maybe<S> for Yes
where
    S::Nullable: Scoped,
{
    type Source = S::Nullable;
    fn scoped(alias: Alias) -> S::Nullable {
        S::Nullable::scoped(alias)
    }
}

/// The source `S`, nullable when the flag `F` is `Yes`.
pub type MaybeNull<S, F> = <F as Maybe<S>>::Source;

impl<T: Table, U, I, D, R: Row> Rows for Merge<T, U, I, D, R> {
    type Row = R;
    fn into_statement(self) -> Statement {
        Statement::Merge(Box::new(self.ast))
    }
}

/// `merge_action()`, the action a MERGE took for a returned row.
pub fn merge_action() -> crate::Expr<Text> {
    crate::Expr::new(Node::Call("merge_action", Vec::new()))
}

/// A value for an inserted column of type `C`: an assignable operand, or
/// `default()`.
pub trait ColumnValue<C> {
    #[doc(hidden)]
    fn into_value(self) -> Node;
}

impl<C, V: Operand> ColumnValue<C> for V
where
    V::Type: Assignable<C>,
{
    fn into_value(self) -> Node {
        self.into_node()
    }
}

/// `DEFAULT`, the column default of an inserted row.
pub struct DefaultValue;

/// `DEFAULT`, for a column of an inserted row.
pub fn default() -> DefaultValue {
    DefaultValue
}

impl<C> ColumnValue<C> for DefaultValue {
    fn into_value(self) -> Node {
        Node::Default
    }
}
