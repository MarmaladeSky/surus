//! Data-modifying statements: `INSERT`, `UPDATE`, `DELETE` and `MERGE`.

use crate::expr::*;
use crate::query::*;
use crate::types::*;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct InsertNode {
    pub table: FromItem,
    pub columns: Vec<&'static str>,
    pub overriding: Option<&'static str>,
    pub source: InsertSource,
    pub on_conflict: Option<OnConflict>,
    pub returning: Vec<Item>,
}

#[derive(Clone, Debug)]
pub enum InsertSource {
    Values(Vec<Vec<Node>>),
    Query(Box<Stmt>),
    DefaultValues,
}

#[derive(Clone, Debug)]
pub struct OnConflict {
    /// Conflict target: `(columns) WHERE ...` or `ON CONSTRAINT name`.
    pub target: Vec<Node>,
    /// `DO UPDATE SET ...`, or `DO NOTHING` when `None`.
    pub update: Option<Vec<Node>>,
    pub filter: Option<Node>,
}

#[derive(Clone, Debug)]
pub struct UpdateNode {
    pub table: FromItem,
    pub set: Vec<Node>,
    pub from: Vec<Join>,
    pub filter: Option<Node>,
    pub returning: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct DeleteNode {
    pub table: FromItem,
    pub using: Vec<Join>,
    pub filter: Option<Node>,
    pub returning: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct MergeNode {
    pub table: FromItem,
    pub source: FromItem,
    pub on: Node,
    pub whens: Vec<Node>,
    pub returning: Vec<Item>,
}

/// Sources visible in a statement modifying table `Q`, besides joined ones.
pub type Target<Q> = (In<Q>, (In<Excluded<Q>>, (In<Old<Q>>, In<New<Q>>)));

/// A table or view: the target of data-modifying statements.
pub trait Table: Requalify {
    /// `INSERT INTO table (columns)`, completed by `values` or `select`.
    fn insert<C: Columns<Self::Id>>(self, columns: C) -> InsertInto<Self::Id, C::Fields> {
        let node = InsertNode {
            table: self.into_from_item(),
            columns: columns.names(),
            overriding: None,
            source: InsertSource::Values(Vec::new()),
            on_conflict: None,
            returning: Vec::new(),
        };
        InsertInto {
            node,
            _type: PhantomData,
        }
    }

    /// `INSERT INTO table DEFAULT VALUES`
    fn insert_default_values(self) -> Insert<Self::Id, (), ()> {
        let mut insert = self.insert(());
        insert.node.source = InsertSource::DefaultValues;
        Insert::new(insert.node)
    }

    /// `UPDATE table SET assignments`
    fn update<A: Assignments<Self::Id>>(self, set: A) -> Update<Target<Self::Id>, A::Scope, ()> {
        let node = UpdateNode {
            table: self.into_from_item(),
            set: set.into_nodes(),
            from: Vec::new(),
            filter: None,
            returning: Vec::new(),
        };
        Update {
            node,
            _type: PhantomData,
        }
    }

    /// `DELETE FROM table`
    fn delete(self) -> Delete<Target<Self::Id>, (), ()> {
        let node = DeleteNode {
            table: self.into_from_item(),
            using: Vec::new(),
            filter: None,
            returning: Vec::new(),
        };
        Delete {
            node,
            _type: PhantomData,
        }
    }

    /// `MERGE INTO table USING source ON condition`
    fn merge<T: Source, P: Predicate>(
        self,
        source: T,
        on: P,
    ) -> Merge<(Target<Self::Id>, In<T::Id>), (T::Scope, P::Scope), ()> {
        let node = MergeNode {
            table: self.into_from_item(),
            source: source.into_from_item(),
            on: on.into_node(),
            whens: Vec::new(),
            returning: Vec::new(),
        };
        Merge {
            node,
            _type: PhantomData,
        }
    }

    /// The `old` row in `RETURNING`; NULL for inserted rows.
    fn old(self) -> Self::As<Old<Self::Id>> {
        self.requalify(Some("old"))
    }

    /// The `new` row in `RETURNING`; NULL for deleted rows.
    fn new(self) -> Self::As<New<Self::Id>> {
        self.requalify(Some("new"))
    }

    /// The row proposed for insertion, in `ON CONFLICT DO UPDATE`.
    fn excluded(self) -> Self::As<Excluded<Self::Id>> {
        self.requalify(Some("excluded"))
    }
}

/// `DEFAULT` as an inserted value.
pub struct Default;

/// `DEFAULT` as an inserted value.
pub fn default() -> Default {
    Default
}

/// A value assignable to a column whose row field is `F` (`T` or `Option<T>`).
pub trait Assign<F> {
    type Scope;
    fn into_value(self) -> Node;
}

impl<F: Field, E: Expression> Assign<F> for E
where
    E::Sql: Coerce<F::Sql>,
    E::Null: Fits<F::Null>,
{
    type Scope = E::Scope;
    fn into_value(self) -> Node {
        self.into_node()
    }
}

impl<F: Field> Assign<F> for Null
where
    Nullable: Fits<F::Null>,
{
    type Scope = ();
    fn into_value(self) -> Node {
        Arg::<F::Sql>::into_arg(self)
    }
}

impl<F: Field> Assign<F> for Placeholder {
    type Scope = ();
    fn into_value(self) -> Node {
        Arg::<F::Sql>::into_arg(self)
    }
}

impl<F> Assign<F> for Default {
    type Scope = ();
    fn into_value(self) -> Node {
        Node::Keyword("DEFAULT")
    }
}

/// Columns of table `Q`, as targets of `INSERT` or a row assignment.
pub trait Columns<Q>: Sized {
    /// The row fields the columns hold.
    type Fields;
    fn names(self) -> Vec<&'static str>;

    /// Row assignment `(columns) = source`, from a subquery or `ROW(...)`.
    fn set<V: RowValue<Self::Fields>>(self, value: V) -> Assignment<Q, V::Scope> {
        let names = self.names().into_iter().map(Node::Ident).collect();
        Assignment::new(Node::Seq(vec![
            Node::List(names),
            Node::Keyword("="),
            value.into_node(),
        ]))
    }
}

impl<Q> Columns<Q> for () {
    type Fields = ();
    fn names(self) -> Vec<&'static str> {
        Vec::new()
    }
}

/// A row of values for columns with fields `F`.
pub trait InsertRow<F> {
    type Scope;
    fn into_nodes(self) -> Vec<Node>;
}

/// A query result row storable in columns with fields `F`.
pub trait RowFits<F> {}

/// A row-valued source for a row assignment.
pub trait RowValue<F> {
    type Scope;
    fn into_node(self) -> Node;
}

impl<F, Q: Statement> RowValue<F> for Q
where
    Q::Row: RowFits<F>,
{
    type Scope = Q::Scope;
    fn into_node(self) -> Node {
        Node::Subquery(Box::new(self.into_stmt()))
    }
}

impl<F, R: RowFits<F> + 'static, S> RowValue<F> for crate::functions::RowExpr<R, S> {
    type Scope = S;
    fn into_node(self) -> Node {
        Expression::into_node(self)
    }
}

/// `column = value` in `SET`.
pub struct Assignment<Q, S> {
    node: Node,
    _type: PhantomData<fn() -> (Q, S)>,
}

impl<Q, S> Assignment<Q, S> {
    fn new(node: Node) -> Self {
        Assignment {
            node,
            _type: PhantomData,
        }
    }
}

impl<T: SqlType, N: Nullability, Q> Column<T, N, Q> {
    /// `column = value` in `SET`.
    pub fn set<V: Assign<N::Field<T>>>(self, value: V) -> Assignment<Q, V::Scope> {
        Assignment::new(Node::Seq(vec![
            Node::Ident(self.name),
            Node::Keyword("="),
            value.into_value(),
        ]))
    }
}

/// One assignment or a tuple of assignments to columns of `Q`.
pub trait Assignments<Q> {
    type Scope;
    fn push_nodes(self, nodes: &mut Vec<Node>);

    fn into_nodes(self) -> Vec<Node>
    where
        Self: Sized,
    {
        let mut nodes = Vec::new();
        self.push_nodes(&mut nodes);
        nodes
    }
}

impl<Q, S> Assignments<Q> for Assignment<Q, S> {
    type Scope = S;
    fn push_nodes(self, nodes: &mut Vec<Node>) {
        nodes.push(self.node);
    }
}

impl<T: SqlType, N: Nullability, Q> Columns<Q> for Column<T, N, Q> {
    type Fields = (N::Field<T>,);
    fn names(self) -> Vec<&'static str> {
        vec![self.name]
    }
}

macro_rules! dml_tuples {
    ($(($($t:ident $i:tt),+))*) => {$(
        impl<Q, $($t: Assignments<Q>),+> Assignments<Q> for ($($t,)+) {
            type Scope = dml_tuples!(@scope $($t::Scope),+);
            fn push_nodes(self, nodes: &mut Vec<Node>) {
                $(self.$i.push_nodes(nodes);)+
            }
        }
    )*};
    (@scope $a:ty) => { $a };
    (@scope $a:ty, $($rest:ty),+) => { ($a, dml_tuples!(@scope $($rest),+)) };
}

for_tuples!(dml_tuples);

/// Implements column lists, value rows and row compatibility for tuples.
macro_rules! column_tuples {
    ($(($($t:ident $v:ident $i:tt),+))*) => {$(
        impl<Q, $($t: Columns<Q, Fields = ($v,)>, $v),+> Columns<Q> for ($($t,)+) {
            type Fields = ($($v,)+);
            fn names(self) -> Vec<&'static str> {
                let mut names = Vec::new();
                $(names.extend(self.$i.names());)+
                names
            }
        }

        impl<$($t: Assign<$v>, $v),+> InsertRow<($($v,)+)> for ($($t,)+) {
            type Scope = dml_tuples!(@scope $($t::Scope),+);
            fn into_nodes(self) -> Vec<Node> {
                vec![$(self.$i.into_value()),+]
            }
        }

        impl<$($t: Field, $v: Field),+> RowFits<($($v,)+)> for ($($t,)+)
        where
            $($t::Sql: Coerce<$v::Sql>, $t::Null: Fits<$v::Null>,)+
        {}
    )*};
}

column_tuples! {
    (A A2 0)
    (A A2 0, B B2 1)
    (A A2 0, B B2 1, C C2 2)
    (A A2 0, B B2 1, C C2 2, D D2 3)
    (A A2 0, B B2 1, C C2 2, D D2 3, E E2 4)
    (A A2 0, B B2 1, C C2 2, D D2 3, E E2 4, F F2 5)
    (A A2 0, B B2 1, C C2 2, D D2 3, E E2 4, F F2 5, G G2 6)
    (A A2 0, B B2 1, C C2 2, D D2 3, E E2 4, F F2 5, G G2 6, H H2 7)
    (A A2 0, B B2 1, C C2 2, D D2 3, E E2 4, F F2 5, G G2 6, H H2 7, I I2 8)
    (A A2 0, B B2 1, C C2 2, D D2 3, E E2 4, F F2 5, G G2 6, H H2 7, I I2 8, J J2 9)
    (A A2 0, B B2 1, C C2 2, D D2 3, E E2 4, F F2 5, G G2 6, H H2 7, I I2 8, J J2 9, K K2 10)
    (A A2 0, B B2 1, C C2 2, D D2 3, E E2 4, F F2 5, G G2 6, H H2 7, I I2 8, J J2 9, K K2 10, L L2 11)
}

// ---------------------------------------------------------------------------
// INSERT

/// `INSERT INTO table (columns)` awaiting its rows.
pub struct InsertInto<Q, C> {
    node: InsertNode,
    _type: PhantomData<fn() -> (Q, C)>,
}

impl<Q, C> InsertInto<Q, C> {
    /// `OVERRIDING SYSTEM VALUE`
    pub fn overriding_system_value(mut self) -> Self {
        self.node.overriding = Some("OVERRIDING SYSTEM VALUE");
        self
    }

    /// `VALUES (row)`; call again on the result for more rows.
    pub fn values<V: InsertRow<C>>(self, row: V) -> Insert<Q, ((), Outside<V::Scope>), (), C> {
        Insert::new(self.node).values(row)
    }

    /// `INSERT ... SELECT`
    pub fn select<S: Statement>(mut self, query: S) -> Insert<Q, Outside<S::Scope>, ()>
    where
        S::Row: RowFits<C>,
    {
        self.node.source = InsertSource::Query(Box::new(query.into_stmt()));
        Insert::new(self.node)
    }
}

/// An `INSERT` into table `Q`, referencing `S`, returning rows `R`, with
/// column fields `C`.
pub struct Insert<Q, S, R, C = ()> {
    node: InsertNode,
    _type: PhantomData<fn() -> (Q, S, R, C)>,
}

impl<Q, S, R, C> Insert<Q, S, R, C> {
    fn new(node: InsertNode) -> Self {
        Insert {
            node,
            _type: PhantomData,
        }
    }

    fn retype<S2, R2>(self) -> Insert<Q, S2, R2, C> {
        Insert::new(self.node)
    }

    /// Another row of `VALUES`.
    pub fn values<V: InsertRow<C>>(mut self, row: V) -> Insert<Q, (S, Outside<V::Scope>), R, C> {
        if let InsertSource::Values(rows) = &mut self.node.source {
            rows.push(row.into_nodes());
        }
        self.retype()
    }

    /// `ON CONFLICT (columns)`
    pub fn on_conflict<K: Columns<Q>>(self, columns: K) -> Conflict<Q, S, R, C, ()> {
        let names = columns.names().into_iter().map(Node::Ident).collect();
        Conflict {
            insert: self,
            target: vec![Node::List(names)],
            _type: PhantomData,
        }
    }

    /// `ON CONFLICT ON CONSTRAINT name`
    pub fn on_conflict_on_constraint(
        self,
        constraint: crate::table::Constraint<Q>,
    ) -> Conflict<Q, S, R, C, ()> {
        let target = vec![Node::Keyword("ON CONSTRAINT"), Node::Ident(constraint.name)];
        Conflict {
            insert: self,
            target,
            _type: PhantomData,
        }
    }

    /// `WHERE condition` of `ON CONFLICT DO UPDATE`.
    pub fn filter<P: Predicate>(mut self, condition: P) -> Insert<Q, (S, P::Scope), R, C> {
        if let Some(conflict) = &mut self.node.on_conflict {
            conflict.filter = Some(condition.into_node());
        }
        self.retype()
    }

    /// `RETURNING projection`
    pub fn returning<P: Projection>(
        mut self,
        projection: P,
    ) -> Insert<Q, (S, P::Scope), P::Row, C> {
        projection.push_items(&mut self.node.returning);
        self.retype()
    }
}

/// `ON CONFLICT target` awaiting its action; `P` is the scope of its
/// index predicate.
pub struct Conflict<Q, S, R, C, P> {
    insert: Insert<Q, S, R, C>,
    target: Vec<Node>,
    _type: PhantomData<fn() -> P>,
}

impl<Q, S, R, C, P> Conflict<Q, S, R, C, P> {
    /// Index predicate of a partial unique index: `WHERE condition`.
    pub fn filter<E: Predicate>(mut self, condition: E) -> Conflict<Q, S, R, C, (P, E::Scope)> {
        self.target
            .extend([Node::Keyword("WHERE"), condition.into_node()]);
        Conflict {
            insert: self.insert,
            target: self.target,
            _type: PhantomData,
        }
    }

    fn action(mut self, update: Option<Vec<Node>>) -> Insert<Q, S, R, C> {
        self.insert.node.on_conflict = Some(OnConflict {
            target: self.target,
            update,
            filter: None,
        });
        self.insert
    }

    /// `DO NOTHING`
    pub fn do_nothing(self) -> Insert<Q, (S, P), R, C> {
        self.action(None).retype()
    }

    /// `DO UPDATE SET assignments`
    pub fn do_update<A: Assignments<Q>>(self, set: A) -> Insert<Q, ((S, P), A::Scope), R, C> {
        let set = set.into_nodes();
        self.action(Some(set)).retype()
    }
}

impl<Q, S, R, C> Statement for Insert<Q, S, R, C> {
    type Row = R;
    type Scope = Sub<S, Target<Q>>;
    fn into_stmt(self) -> Stmt {
        Stmt::Insert(Box::new(self.node))
    }
}

// ---------------------------------------------------------------------------
// UPDATE, DELETE

/// An `UPDATE` with sources `F`, referencing `S`, returning rows `R`.
pub struct Update<F, S, R> {
    node: UpdateNode,
    _type: PhantomData<fn() -> (F, S, R)>,
}

impl<F, S, R> Update<F, S, R> {
    fn retype<F2, S2, R2>(self) -> Update<F2, S2, R2> {
        Update {
            node: self.node,
            _type: PhantomData,
        }
    }

    /// `FROM source`
    pub fn from<T: Source>(mut self, source: T) -> Update<(F, In<T::Id>), (S, T::Scope), R> {
        let kind = if self.node.from.is_empty() { "" } else { "," };
        self.node.from.push(Join {
            kind,
            item: source.into_from_item(),
            condition: None,
        });
        self.retype()
    }

    /// Adds a `WHERE` condition.
    pub fn filter<P: Predicate>(mut self, condition: P) -> Update<F, (S, P::Scope), R> {
        self.node.filter = Some(match self.node.filter.take() {
            Some(left) => Node::binary("AND", left, condition.into_node()),
            None => condition.into_node(),
        });
        self.retype()
    }

    /// `RETURNING projection`
    pub fn returning<P: Projection>(mut self, projection: P) -> Update<F, (S, P::Scope), P::Row> {
        projection.push_items(&mut self.node.returning);
        self.retype()
    }
}

impl<F, S, R> Statement for Update<F, S, R> {
    type Row = R;
    type Scope = Sub<S, F>;
    fn into_stmt(self) -> Stmt {
        Stmt::Update(Box::new(self.node))
    }
}

/// A `DELETE` with sources `F`, referencing `S`, returning rows `R`.
pub struct Delete<F, S, R> {
    node: DeleteNode,
    _type: PhantomData<fn() -> (F, S, R)>,
}

impl<F, S, R> Delete<F, S, R> {
    fn retype<F2, S2, R2>(self) -> Delete<F2, S2, R2> {
        Delete {
            node: self.node,
            _type: PhantomData,
        }
    }

    /// `USING source`
    pub fn using<T: Source>(mut self, source: T) -> Delete<(F, In<T::Id>), (S, T::Scope), R> {
        let kind = if self.node.using.is_empty() { "" } else { "," };
        self.node.using.push(Join {
            kind,
            item: source.into_from_item(),
            condition: None,
        });
        self.retype()
    }

    /// Adds a `WHERE` condition.
    pub fn filter<P: Predicate>(mut self, condition: P) -> Delete<F, (S, P::Scope), R> {
        self.node.filter = Some(match self.node.filter.take() {
            Some(left) => Node::binary("AND", left, condition.into_node()),
            None => condition.into_node(),
        });
        self.retype()
    }

    /// `RETURNING projection`
    pub fn returning<P: Projection>(mut self, projection: P) -> Delete<F, (S, P::Scope), P::Row> {
        projection.push_items(&mut self.node.returning);
        self.retype()
    }
}

impl<F, S, R> Statement for Delete<F, S, R> {
    type Row = R;
    type Scope = Sub<S, F>;
    fn into_stmt(self) -> Stmt {
        Stmt::Delete(Box::new(self.node))
    }
}

// ---------------------------------------------------------------------------
// MERGE

/// A `MERGE` with sources `F`, referencing `S`, returning rows `R`.
pub struct Merge<F, S, R> {
    node: MergeNode,
    _type: PhantomData<fn() -> (F, S, R)>,
}

/// The action of a `MERGE` `WHEN` clause on table `Q`.
pub struct MergeAction<Q, S> {
    node: Node,
    _type: PhantomData<fn() -> (Q, S)>,
}

impl<Q, S> MergeAction<Q, S> {
    fn new(parts: Vec<Node>) -> Self {
        MergeAction {
            node: Node::Seq(parts),
            _type: PhantomData,
        }
    }
}

/// `UPDATE SET assignments` in `MERGE`.
pub fn update<Q, A: Assignments<Q>>(set: A) -> MergeAction<Q, A::Scope> {
    let mut parts = vec![Node::Keyword("UPDATE SET")];
    for (i, assignment) in set.into_nodes().into_iter().enumerate() {
        if i > 0 {
            parts.push(Node::Keyword(","));
        }
        parts.push(assignment);
    }
    MergeAction::new(parts)
}

/// `INSERT (columns) VALUES (row)` in `MERGE`.
pub fn insert<Q, C: Columns<Q>, V: InsertRow<C::Fields>>(
    columns: C,
    row: V,
) -> MergeAction<Q, V::Scope> {
    let names = columns.names().into_iter().map(Node::Ident).collect();
    MergeAction::new(vec![
        Node::Keyword("INSERT"),
        Node::List(names),
        Node::Keyword("VALUES"),
        Node::List(row.into_nodes()),
    ])
}

/// `DELETE` in `MERGE`.
pub fn delete<Q>() -> MergeAction<Q, ()> {
    MergeAction::new(vec![Node::Keyword("DELETE")])
}

/// `DO NOTHING` in `MERGE`.
pub fn do_nothing<Q>() -> MergeAction<Q, ()> {
    MergeAction::new(vec![Node::Keyword("DO NOTHING")])
}

impl<Q, F, S, R> Merge<(Target<Q>, F), S, R> {
    fn when<S2>(
        mut self,
        clause: &'static str,
        action: MergeAction<Q, S2>,
    ) -> Merge<(Target<Q>, F), (S, S2), R> {
        self.node
            .whens
            .push(Node::Seq(vec![Node::Keyword(clause), action.node]));
        Merge {
            node: self.node,
            _type: PhantomData,
        }
    }

    /// `WHEN MATCHED THEN action`
    pub fn when_matched<S2>(self, action: MergeAction<Q, S2>) -> Merge<(Target<Q>, F), (S, S2), R> {
        self.when("WHEN MATCHED THEN", action)
    }

    /// `WHEN NOT MATCHED THEN action`
    pub fn when_not_matched<S2>(
        self,
        action: MergeAction<Q, S2>,
    ) -> Merge<(Target<Q>, F), (S, S2), R> {
        self.when("WHEN NOT MATCHED THEN", action)
    }

    /// `WHEN NOT MATCHED BY SOURCE THEN action`
    pub fn when_not_matched_by_source<S2>(
        self,
        action: MergeAction<Q, S2>,
    ) -> Merge<(Target<Q>, F), (S, S2), R> {
        self.when("WHEN NOT MATCHED BY SOURCE THEN", action)
    }

    /// `RETURNING projection`
    pub fn returning<P: Projection>(
        mut self,
        projection: P,
    ) -> Merge<(Target<Q>, F), (S, P::Scope), P::Row> {
        projection.push_items(&mut self.node.returning);
        Merge {
            node: self.node,
            _type: PhantomData,
        }
    }
}

impl<F, S, R> Statement for Merge<F, S, R> {
    type Row = R;
    type Scope = Sub<S, F>;
    fn into_stmt(self) -> Stmt {
        Stmt::Merge(Box::new(self.node))
    }
}
