//! Queries: FROM items, SELECT, set operations, VALUES and CTEs.

use crate::ast::{self, Join, Node, Rel, Statement};
use crate::expr::{Aliased, Base, Column, Expr, Expression, InList, Operand, Operands, Order};
use crate::schema::{Alias, Expand, Scoped, Star};
use crate::types::*;
use crate::{Row, ops};
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// A process-wide unique id for FROM items, CTEs and groupings, so that
/// nested queries never confuse their sources.
pub(crate) fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// What a FROM item hands to the closures of a query: a table struct, or a
/// tuple of columns for subqueries, CTEs, functions and VALUES.
pub trait Source: Clone {
    /// The same source on the nullable side of an outer join.
    type Nullable: Source;
    fn nullable(&self) -> Self::Nullable;
}

impl<T: PgType> Source for Column<T> {
    type Nullable = Column<Nullable<T::Base>>;
    fn nullable(&self) -> Self::Nullable {
        Column::nullable(*self)
    }
}

/// The sources in scope of a query, a tuple; closures receive the `View`,
/// the single source itself or the tuple of sources of a join.
pub trait Sources: Clone {
    type View: Clone;
    fn view(&self) -> Self::View;
}

impl Sources for () {
    type View = ();
    fn view(&self) {}
}

/// Adds a joined source to a scope.
pub trait Append<T> {
    type Out: Sources;
    fn append(self, source: T) -> Self::Out;
}

/// The scope after GROUP BY ROLLUP, CUBE or GROUPING SETS: the sources and
/// the grouping keys, which are NULL in rows of super-aggregates.
#[derive(Clone)]
pub struct Grouped<S, K>(S, K);

impl<S: Sources, K: Clone> Sources for Grouped<S, K> {
    type View = (S::View, K);
    fn view(&self) -> Self::View {
        (self.0.view(), self.1.clone())
    }
}

/// A statement that produces rows of type `Row`.
pub trait Rows {
    type Row: Row;
    #[doc(hidden)]
    fn into_statement(self) -> Statement;
}

/// Something that can appear in FROM.
pub trait FromItem {
    type Source: Source;
    #[doc(hidden)]
    fn into_from(self) -> (ast::From, Self::Source);
}

fn positional<R: Row>(source: ast::Source) -> (ast::From, R::Columns) {
    let id = next_id();
    let from = ast::From {
        source,
        rel: id,
        columns: R::LEN,
    };
    (from, R::columns(Alias(Rel::Source(id))))
}

fn subquery<Q: Rows>(query: Q) -> (ast::From, <Q::Row as Row>::Columns) {
    positional::<Q::Row>(ast::Source::Query(query.into_statement()))
}

/// Starts a query from a FROM item.
pub fn from<F: FromItem>(item: F) -> Select<(F::Source,)> {
    let (from, source) = item.into_from();
    let ast = ast::Select {
        from: vec![Join {
            kind: "",
            lateral: false,
            from,
            on: None,
        }],
        ..Default::default()
    };
    Select {
        scope: (source,),
        ast,
        row: PhantomData,
    }
}

/// A SELECT without FROM.
pub fn select<P: Projection>(projection: P) -> Select<(), P::Row> {
    let mut ast = ast::Select::default();
    projection.items(&mut ast.items);
    Select {
        scope: (),
        ast,
        row: PhantomData,
    }
}

/// A SELECT statement in scope `S` with row type `R`, `()` before `select`.
pub struct Select<S, R = ()> {
    scope: S,
    ast: ast::Select,
    row: PhantomData<fn() -> R>,
}

fn and(filter: &mut Option<Node>, node: Node) {
    *filter = Some(match filter.take() {
        Some(previous) => Node::binary("AND", previous, node),
        None => node,
    });
}

impl<S: Sources> Select<S> {
    fn push<T, O: Sources>(
        mut self,
        kind: &'static str,
        lateral: bool,
        from: ast::From,
        source: T,
    ) -> Select<O>
    where
        S: Append<T, Out = O>,
    {
        self.ast.from.push(Join {
            kind,
            lateral,
            from,
            on: None,
        });
        Select {
            scope: self.scope.append(source),
            ast: self.ast,
            row: PhantomData,
        }
    }

    fn on<E: Operand>(mut self, on: impl FnOnce(S::View) -> E) -> Self
    where
        Base<E>: Boolean,
    {
        let node = on(self.scope.view()).into_node();
        self.ast.from.last_mut().unwrap().on = Some(node);
        self
    }

    /// `JOIN item ON condition`.
    pub fn join<F: FromItem, E: Operand, O: Sources>(
        self,
        item: F,
        on: impl FnOnce(O::View) -> E,
    ) -> Select<O>
    where
        S: Append<F::Source, Out = O>,
        Base<E>: Boolean,
    {
        let (from, source) = item.into_from();
        self.push("JOIN", false, from, source).on(on)
    }

    /// `LEFT JOIN item ON condition`: the item's columns become nullable.
    pub fn left_join<F: FromItem, E: Operand, O: Sources>(
        self,
        item: F,
        on: impl FnOnce(O::View) -> E,
    ) -> Select<O>
    where
        S: Append<<F::Source as Source>::Nullable, Out = O>,
        Base<E>: Boolean,
    {
        let (from, source) = item.into_from();
        self.push("LEFT JOIN", false, from, source.nullable())
            .on(on)
    }

    /// `RIGHT JOIN item ON condition`: the columns joined so far become
    /// nullable.
    pub fn right_join<F: FromItem, E: Operand, O: Sources>(
        self,
        item: F,
        on: impl FnOnce(O::View) -> E,
    ) -> Select<O>
    where
        S: Source,
        S::Nullable: Sources + Append<F::Source, Out = O>,
        Base<E>: Boolean,
    {
        let (from, source) = item.into_from();
        self.nullable()
            .push("RIGHT JOIN", false, from, source)
            .on(on)
    }

    /// `FULL JOIN item ON condition`: all columns become nullable.
    pub fn full_join<F: FromItem, E: Operand, O: Sources>(
        self,
        item: F,
        on: impl FnOnce(O::View) -> E,
    ) -> Select<O>
    where
        S: Source,
        S::Nullable: Sources + Append<<F::Source as Source>::Nullable, Out = O>,
        Base<E>: Boolean,
    {
        let (from, source) = item.into_from();
        self.nullable()
            .push("FULL JOIN", false, from, source.nullable())
            .on(on)
    }

    fn nullable(self) -> Select<S::Nullable>
    where
        S: Source,
        S::Nullable: Sources,
    {
        Select {
            scope: self.scope.nullable(),
            ast: self.ast,
            row: PhantomData,
        }
    }

    /// `CROSS JOIN item`.
    pub fn cross_join<F: FromItem>(self, item: F) -> Select<S::Out>
    where
        S: Append<F::Source>,
    {
        let (from, source) = item.into_from();
        self.push("CROSS JOIN", false, from, source)
    }

    /// `JOIN LATERAL item ON condition`, the item built from the sources so far.
    pub fn join_lateral<F: FromItem, E: Operand, O: Sources>(
        self,
        item: impl FnOnce(S::View) -> F,
        on: impl FnOnce(O::View) -> E,
    ) -> Select<O>
    where
        S: Append<F::Source, Out = O>,
        Base<E>: Boolean,
    {
        let (from, source) = item(self.scope.view()).into_from();
        self.push("JOIN", true, from, source).on(on)
    }

    /// `LEFT JOIN LATERAL item ON condition`.
    pub fn left_join_lateral<F: FromItem, E: Operand, O: Sources>(
        self,
        item: impl FnOnce(S::View) -> F,
        on: impl FnOnce(O::View) -> E,
    ) -> Select<O>
    where
        S: Append<<F::Source as Source>::Nullable, Out = O>,
        Base<E>: Boolean,
    {
        let (from, source) = item(self.scope.view()).into_from();
        self.push("LEFT JOIN", true, from, source.nullable()).on(on)
    }

    /// `CROSS JOIN LATERAL item`.
    pub fn cross_join_lateral<F: FromItem>(self, item: impl FnOnce(S::View) -> F) -> Select<S::Out>
    where
        S: Append<F::Source>,
    {
        let (from, source) = item(self.scope.view()).into_from();
        self.push("CROSS JOIN", true, from, source)
    }

    /// `GROUP BY`: a key, a tuple of keys, or `rollup`, `cube` and
    /// `grouping_sets`, after which closures also receive the nullable keys.
    pub fn group_by<G: Grouping>(mut self, keys: impl FnOnce(S::View) -> G) -> Select<G::Scope<S>> {
        let (group, scope) = keys(self.scope.view()).into_group(next_id(), self.scope);
        self.ast.group = Some(group);
        Select {
            scope,
            ast: self.ast,
            row: PhantomData,
        }
    }

    /// The SELECT list.
    pub fn select<P: Projection>(
        mut self,
        projection: impl FnOnce(S::View) -> P,
    ) -> Select<S, P::Row> {
        projection(self.scope.view()).items(&mut self.ast.items);
        Select {
            scope: self.scope,
            ast: self.ast,
            row: PhantomData,
        }
    }

    pub(crate) fn into_parts(self) -> (S, ast::Select) {
        (self.scope, self.ast)
    }
}

impl<S: Sources, R> Select<S, R> {
    /// `WHERE condition`; several filters are combined with AND.
    pub fn filter<E: Operand>(mut self, condition: impl FnOnce(S::View) -> E) -> Self
    where
        Base<E>: Boolean,
    {
        let node = condition(self.scope.view()).into_node();
        and(&mut self.ast.filter, node);
        self
    }

    /// `HAVING condition`.
    pub fn having<E: Operand>(mut self, condition: impl FnOnce(S::View) -> E) -> Self
    where
        Base<E>: Boolean,
    {
        let node = condition(self.scope.view()).into_node();
        and(&mut self.ast.having, node);
        self
    }

    /// `ORDER BY`: an expression, an `asc`/`desc` order, or a tuple of them.
    pub fn order_by<O: OrderBy>(mut self, order: impl FnOnce(S::View) -> O) -> Self {
        order(self.scope.view()).orders(&mut self.ast.order);
        self
    }

    pub fn distinct(mut self) -> Self {
        self.ast.distinct = Some(Vec::new());
        self
    }

    /// `DISTINCT ON (expressions)`.
    pub fn distinct_on<E: Exprs>(mut self, on: impl FnOnce(S::View) -> E) -> Self {
        self.ast.distinct = Some(on(self.scope.view()).into_nodes());
        self
    }

    pub fn limit<N: Operand>(mut self, count: N) -> Self
    where
        Base<N>: Coerce<Int8>,
    {
        self.ast.limit = Some(count.into_node());
        self
    }

    pub fn offset<N: Operand>(mut self, count: N) -> Self
    where
        Base<N>: Coerce<Int8>,
    {
        self.ast.offset = Some(count.into_node());
        self
    }

    /// `FETCH FIRST count ROWS ONLY`.
    pub fn fetch_first<N: Operand>(mut self, count: N) -> Self
    where
        Base<N>: Coerce<Int8>,
    {
        self.ast.fetch = Some((count.into_node(), false));
        self
    }

    /// `FETCH FIRST count ROWS WITH TIES`.
    pub fn fetch_first_with_ties<N: Operand>(mut self, count: N) -> Self
    where
        Base<N>: Coerce<Int8>,
    {
        self.ast.fetch = Some((count.into_node(), true));
        self
    }

    fn lock(mut self, strength: &'static str) -> Self {
        self.ast.locks.push(ast::Lock {
            strength,
            of: Vec::new(),
            wait: None,
        });
        self
    }

    pub fn for_update(self) -> Self {
        self.lock("UPDATE")
    }

    pub fn for_no_key_update(self) -> Self {
        self.lock("NO KEY UPDATE")
    }

    pub fn for_share(self) -> Self {
        self.lock("SHARE")
    }

    pub fn for_key_share(self) -> Self {
        self.lock("KEY SHARE")
    }

    /// Restricts the last locking clause to a table: `FOR UPDATE OF t`.
    pub fn of<T: Scoped>(mut self, table: impl FnOnce(S::View) -> T) -> Self {
        let Rel::Source(id) = table(self.scope.view()).alias().0 else {
            unreachable!("tables are FROM items")
        };
        self.last_lock().of.push(id);
        self
    }

    pub fn skip_locked(mut self) -> Self {
        self.last_lock().wait = Some("SKIP LOCKED");
        self
    }

    pub fn nowait(mut self) -> Self {
        self.last_lock().wait = Some("NOWAIT");
        self
    }

    fn last_lock(&mut self) -> &mut ast::Lock {
        self.ast
            .locks
            .last_mut()
            .expect("a locking clause such as for_update comes first")
    }
}

macro_rules! set_operations {
    ($($method:ident $op:literal)*) => {$(
        #[doc = concat!("`", $op, "`, with rows of the same column types.")]
        pub fn $method<Q: Rows>(self, other: Q) -> Compound<<Q::Row as UnionRow<R>>::Out>
        where
            Q::Row: UnionRow<R>,
        {
            Compound::new($op, self.into_statement(), other.into_statement())
        }
    )*};
}

impl<S: Sources, R: Row> Select<S, R> {
    /// `ORDER BY` output columns, which the closure receives; rendered as
    /// positions, as needed for set-returning functions.
    pub fn order_by_output<O: OrderBy>(mut self, order: impl FnOnce(R::Columns) -> O) -> Self {
        order(R::outputs()).orders(&mut self.ast.order);
        self
    }

    set_operations! {
        union "UNION"
        union_all "UNION ALL"
        intersect "INTERSECT"
        intersect_all "INTERSECT ALL"
        except "EXCEPT"
        except_all "EXCEPT ALL"
    }
}

impl<S, R: Row> Rows for Select<S, R> {
    type Row = R;
    fn into_statement(self) -> Statement {
        Statement::Select(Box::new(self.ast))
    }
}

impl<S, R: Row> FromItem for Select<S, R> {
    type Source = R::Columns;
    fn into_from(self) -> (ast::From, R::Columns) {
        subquery(self)
    }
}

/// A scalar subquery: a single-column query used as a value, NULL when it
/// returns no row.
impl<S, T: PgType> Operand for Select<S, (T,)> {
    type Type = Nullable<T::Base>;
    fn into_node(self) -> Node {
        Node::Subquery(Box::new(self.into_statement()))
    }
}

/// A row type compatible with the row type `L` in a set operation: of the
/// same column types, nullable when nullable on either side.
pub trait UnionRow<L> {
    type Out: Row;
}

/// A set operation: UNION, INTERSECT or EXCEPT.
pub struct Compound<R> {
    ast: ast::Compound,
    row: PhantomData<fn() -> R>,
}

impl<R: Row> Compound<R> {
    fn new(op: &'static str, left: Statement, right: Statement) -> Compound<R> {
        Compound {
            ast: ast::Compound {
                op,
                left,
                right,
                order: Vec::new(),
                limit: None,
                offset: None,
            },
            row: PhantomData,
        }
    }

    set_operations! {
        union "UNION"
        union_all "UNION ALL"
        intersect "INTERSECT"
        intersect_all "INTERSECT ALL"
        except "EXCEPT"
        except_all "EXCEPT ALL"
    }

    /// `ORDER BY` output columns, which the closure receives.
    pub fn order_by<O: OrderBy>(mut self, order: impl FnOnce(R::Columns) -> O) -> Self {
        order(R::outputs()).orders(&mut self.ast.order);
        self
    }

    pub fn limit<N: Operand>(mut self, count: N) -> Self
    where
        Base<N>: Coerce<Int8>,
    {
        self.ast.limit = Some(count.into_node());
        self
    }

    pub fn offset<N: Operand>(mut self, count: N) -> Self
    where
        Base<N>: Coerce<Int8>,
    {
        self.ast.offset = Some(count.into_node());
        self
    }
}

impl<R: Row> Rows for Compound<R> {
    type Row = R;
    fn into_statement(self) -> Statement {
        Statement::Compound(Box::new(self.ast))
    }
}

impl<R: Row> FromItem for Compound<R> {
    type Source = R::Columns;
    fn into_from(self) -> (ast::From, R::Columns) {
        subquery(self)
    }
}

impl<T: PgType> Operand for Compound<(T,)> {
    type Type = Nullable<T::Base>;
    fn into_node(self) -> Node {
        Node::Subquery(Box::new(self.into_statement()))
    }
}

/// A VALUES list.
pub struct Values<R> {
    rows: Vec<Vec<Node>>,
    row: PhantomData<fn() -> R>,
}

/// `VALUES (row)`; add rows with `row`.
pub fn values<V: Operands>(row: V) -> Values<V::Types>
where
    V::Types: Row,
{
    Values {
        rows: vec![row.into_nodes()],
        row: PhantomData,
    }
}

impl<R: Row> Values<R> {
    pub fn row<V: Operands>(mut self, row: V) -> Values<<V::Types as UnionRow<R>>::Out>
    where
        V::Types: UnionRow<R>,
    {
        self.rows.push(row.into_nodes());
        Values {
            rows: self.rows,
            row: PhantomData,
        }
    }
}

impl<R: Row> Rows for Values<R> {
    type Row = R;
    fn into_statement(self) -> Statement {
        Statement::Values(self.rows)
    }
}

impl<R: Row> FromItem for Values<R> {
    type Source = R::Columns;
    fn into_from(self) -> (ast::From, R::Columns) {
        subquery(self)
    }
}

/// A common table expression, defined in the WITH clause of the statement
/// that uses it.
pub struct Cte<R> {
    id: u64,
    definition: Option<Arc<ast::Cte>>,
    row: PhantomData<fn() -> R>,
}

impl<R> Clone for Cte<R> {
    fn clone(&self) -> Self {
        Cte {
            id: self.id,
            definition: self.definition.clone(),
            row: PhantomData,
        }
    }
}

/// `WITH name AS (query)`.
pub fn cte<Q: Rows>(query: Q) -> Cte<Q::Row> {
    let id = next_id();
    Cte {
        id,
        definition: Some(Arc::new(ast::Cte {
            id,
            body: query.into_statement(),
            columns: Q::Row::LEN,
            recursive: false,
            materialized: None,
        })),
        row: PhantomData,
    }
}

/// `WITH RECURSIVE name AS (base UNION ALL step)`, the step built from the
/// CTE itself; both produce rows of the same type.
pub fn recursive<B: Rows, Q: Rows>(base: B, step: impl FnOnce(Cte<B::Row>) -> Q) -> Cte<B::Row>
where
    Q::Row: SameType<B::Row>,
{
    let id = next_id();
    let step = step(Cte {
        id,
        definition: None,
        row: PhantomData,
    });
    let body = Compound::<B::Row>::new("UNION ALL", base.into_statement(), step.into_statement());
    Cte {
        id,
        definition: Some(Arc::new(ast::Cte {
            id,
            body: body.into_statement(),
            columns: B::Row::LEN,
            recursive: true,
            materialized: None,
        })),
        row: PhantomData,
    }
}

impl<R> Cte<R> {
    fn materialize(mut self, materialized: bool) -> Self {
        let definition = self.definition.take().expect("a defined CTE");
        let mut definition = Arc::try_unwrap(definition).expect("a CTE not used yet");
        definition.materialized = Some(materialized);
        self.definition = Some(Arc::new(definition));
        self
    }

    /// `AS MATERIALIZED`.
    pub fn materialized(self) -> Self {
        self.materialize(true)
    }

    /// `AS NOT MATERIALIZED`.
    pub fn not_materialized(self) -> Self {
        self.materialize(false)
    }
}

impl<R: Row> FromItem for Cte<R> {
    type Source = R::Columns;
    fn into_from(self) -> (ast::From, R::Columns) {
        let source = match self.definition {
            Some(definition) => ast::Source::Cte(definition),
            None => ast::Source::Recursive(self.id),
        };
        let (mut from, columns) = positional::<R>(source);
        from.columns = 0;
        (from, columns)
    }
}

/// The SELECT list: an expression, an aliased expression, `source.star()`,
/// or a tuple of them.
pub trait Projection {
    type Row: Row;
    #[doc(hidden)]
    fn items(self, items: &mut Vec<ast::Item>);
}

impl<E: Operand> Projection for E {
    type Row = (E::Type,);
    fn items(self, items: &mut Vec<ast::Item>) {
        items.push(ast::Item {
            node: self.into_node(),
            alias: None,
        });
    }
}

impl<E: Operand> Projection for Aliased<E> {
    type Row = (E::Type,);
    fn items(self, items: &mut Vec<ast::Item>) {
        items.push(ast::Item {
            node: self.expr.into_node(),
            alias: Some(self.name),
        });
    }
}

impl<R: Row> Projection for Star<R> {
    type Row = R;
    fn items(self, items: &mut Vec<ast::Item>) {
        items.push(ast::Item {
            node: Node::Star(self.0),
            alias: None,
        });
    }
}

impl<R: Row> Projection for Expand<R> {
    type Row = R;
    fn items(self, items: &mut Vec<ast::Item>) {
        items.push(ast::Item {
            node: self.0,
            alias: None,
        });
    }
}

/// An ORDER BY list: an expression, an `Order`, or a tuple of them.
pub trait OrderBy {
    #[doc(hidden)]
    fn orders(self, orders: &mut Vec<ast::Order>);
}

impl OrderBy for Order {
    fn orders(self, orders: &mut Vec<ast::Order>) {
        orders.push(self.0);
    }
}

impl<E: Expression> OrderBy for E {
    fn orders(self, orders: &mut Vec<ast::Order>) {
        orders.push(ast::Order {
            node: self.into_node(),
            direction: None,
            nulls: None,
        });
    }
}

/// A list of expressions: one, or a tuple.
pub trait Exprs {
    #[doc(hidden)]
    fn into_nodes(self) -> Vec<Node>;
}

impl<E: Operand> Exprs for E {
    fn into_nodes(self) -> Vec<Node> {
        vec![self.into_node()]
    }
}

/// A GROUP BY clause.
pub trait Grouping {
    /// The scope of the rest of the query.
    type Scope<S: Sources>: Sources;
    #[doc(hidden)]
    fn into_group<S: Sources>(self, id: u64, scope: S) -> (ast::Group, Self::Scope<S>);
}

impl<E: Operand> Grouping for E {
    type Scope<S: Sources> = S;
    fn into_group<S: Sources>(self, id: u64, scope: S) -> (ast::Group, S) {
        let group = ast::Group {
            id,
            kind: "",
            keys: vec![self.into_node()],
            sets: Vec::new(),
        };
        (group, scope)
    }
}

/// Grouping keys that become NULL in super-aggregate rows.
pub struct Sets<K> {
    kind: &'static str,
    keys: K,
    sets: Vec<Vec<usize>>,
}

/// `ROLLUP (keys)`.
pub fn rollup<K: Operands>(keys: K) -> Sets<K> {
    Sets {
        kind: "ROLLUP",
        keys,
        sets: Vec::new(),
    }
}

/// `CUBE (keys)`.
pub fn cube<K: Operands>(keys: K) -> Sets<K> {
    Sets {
        kind: "CUBE",
        keys,
        sets: Vec::new(),
    }
}

/// `GROUPING SETS`, of the keys picked by the closure from `keys`:
/// `grouping_sets((a, b), |(a, b)| ((a,), (b,), ()))`.
pub fn grouping_sets<K: Operands, G: KeySets>(
    keys: K,
    sets: impl FnOnce(<K::Types as Row>::Columns) -> G,
) -> Sets<K>
where
    K::Types: Row,
{
    let handles = <K::Types as Row>::keys(0);
    Sets {
        kind: "GROUPING SETS",
        keys,
        sets: sets(handles).sets(),
    }
}

impl<K: Operands> Grouping for Sets<K>
where
    K::Types: Row,
{
    type Scope<S: Sources> = Grouped<S, <<K::Types as Row>::Columns as Source>::Nullable>;
    fn into_group<S: Sources>(self, id: u64, scope: S) -> (ast::Group, Self::Scope<S>) {
        let group = ast::Group {
            id,
            kind: self.kind,
            keys: self.keys.into_nodes(),
            sets: self.sets,
        };
        (
            group,
            Grouped(scope, <K::Types as Row>::keys(id).nullable()),
        )
    }
}

/// Grouping sets: a tuple of tuples of grouping keys.
pub trait KeySets {
    #[doc(hidden)]
    fn sets(self) -> Vec<Vec<usize>>;
}

/// One grouping set: a tuple of grouping keys.
pub trait KeySet {
    #[doc(hidden)]
    fn set(self) -> Vec<usize>;
}

pub(crate) fn key_index<T: PgType>(column: &Column<T>) -> usize {
    match column.rel() {
        Rel::Key(_, index) => index,
        _ => panic!("grouping sets are made of grouping keys"),
    }
}

/// A single-column row.
pub trait Single {
    type Type: PgType;
}

impl<T: PgType> Single for (T,) {
    type Type = T;
}

impl<T, Q: Rows> InList<T> for Q
where
    Q::Row: Single,
    T: ops::Eq<<<Q::Row as Single>::Type as PgType>::Base>,
{
    type Null = IsNull;
    fn into_list(self) -> Node {
        Node::Subquery(Box::new(self.into_statement()))
    }
}

/// `EXISTS (query)`.
pub fn exists<Q: Rows>(query: Q) -> Expr<Bool> {
    Expr::new(Node::Prefix(
        "EXISTS",
        Box::new(Node::Subquery(Box::new(query.into_statement()))),
    ))
}

/// A quantified operand, `ANY (...)` or `ALL (...)`, compared element-wise.
pub struct Quantified<T> {
    node: Node,
    ty: PhantomData<fn() -> T>,
}

impl<T: PgType<Null = NotNull>> Operand for Quantified<T> {
    type Type = Nullable<T>;
    fn into_node(self) -> Node {
        self.node
    }
}

fn quantified<T>(quantifier: &'static str, node: Node) -> Quantified<T> {
    Quantified {
        node: Node::Quantified(quantifier, Box::new(node)),
        ty: PhantomData,
    }
}

/// `ANY (subquery)`, as in `x.gt(any(query))`.
pub fn any<Q: Rows>(query: Q) -> Quantified<<<Q::Row as Single>::Type as PgType>::Base>
where
    Q::Row: Single,
{
    quantified("ANY", Node::Subquery(Box::new(query.into_statement())))
}

/// `ALL (subquery)`.
pub fn all<Q: Rows>(query: Q) -> Quantified<<<Q::Row as Single>::Type as PgType>::Base>
where
    Q::Row: Single,
{
    quantified("ALL", Node::Subquery(Box::new(query.into_statement())))
}

/// `ANY (array)`, as in `x.eq(any_of(ids))`.
pub fn any_of<A: Operand>(array: A) -> Quantified<<Base<A> as crate::Indexed>::Element>
where
    Base<A>: crate::Indexed,
{
    quantified("ANY", array.into_node())
}

/// `ALL (array)`.
pub fn all_of<A: Operand>(array: A) -> Quantified<<Base<A> as crate::Indexed>::Element>
where
    Base<A>: crate::Indexed,
{
    quantified("ALL", array.into_node())
}
