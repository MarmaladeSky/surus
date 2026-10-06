//! Queries: sources, `SELECT`, set operations, `VALUES` and CTEs.
//!
//! A query's Rust type records the sources of its `FROM` clause (`F`), the
//! sources referenced by its expressions (`S`) and its result row (`R`).
//! [`Statement::compile`] requires every referenced source to be in scope.

use crate::dml::{DeleteNode, InsertNode, MergeNode, UpdateNode};
use crate::expr::rust_operators;
use crate::expr::*;
use crate::render::Compiled;
use crate::types::*;
use std::marker::PhantomData;
use std::rc::Rc;

/// Statement syntax tree.
#[derive(Clone, Debug)]
pub enum Stmt {
    Select(Box<SelectNode>),
    SetOp(Box<SetOpNode>),
    Values(Vec<Vec<Node>>),
    Insert(Box<InsertNode>),
    Update(Box<UpdateNode>),
    Delete(Box<DeleteNode>),
    Merge(Box<MergeNode>),
}

#[derive(Clone, Debug, Default)]
pub struct SelectNode {
    /// `Some(vec![])` for `DISTINCT`, otherwise `DISTINCT ON (...)`.
    pub distinct: Option<Vec<Node>>,
    pub items: Vec<Item>,
    pub from: Vec<Join>,
    pub filter: Option<Node>,
    pub group: Vec<Node>,
    pub having: Option<Node>,
    pub windows: Vec<(&'static str, Window)>,
    pub order: Vec<Node>,
    pub limit: Option<Node>,
    pub offset: Option<Node>,
    /// `FETCH FIRST n ROWS ONLY`, or `WITH TIES` when the flag is set.
    pub fetch: Option<(Node, bool)>,
    pub locks: Vec<Vec<Node>>,
}

/// A projected expression with an optional output name.
#[derive(Clone, Debug)]
pub struct Item {
    pub node: Node,
    pub alias: Option<&'static str>,
}

impl Item {
    /// The output column name, when known.
    pub fn name(&self) -> Option<&'static str> {
        match (&self.node, self.alias) {
            (_, Some(alias)) => Some(alias),
            (Node::Column(_, name), None) if *name != "*" => Some(name),
            _ => None,
        }
    }
}

/// A `FROM` item with the join that attaches it; the first has no join.
#[derive(Clone, Debug)]
pub struct Join {
    pub kind: &'static str,
    pub item: FromItem,
    /// `ON ...` or `USING (...)`.
    pub condition: Option<Node>,
}

#[derive(Clone, Debug)]
pub struct FromItem {
    pub kind: FromKind,
    pub alias: Option<&'static str>,
    pub columns: Vec<&'static str>,
    pub lateral: bool,
    pub sample: Option<Node>,
}

#[derive(Clone, Debug)]
pub enum FromKind {
    Table(&'static str, &'static str),
    Query(Box<Stmt>),
    Function(Node),
    Cte(Rc<CteDef>),
    /// A recursive CTE referenced from its own definition.
    CteSelf(&'static str),
}

impl FromItem {
    pub(crate) fn new(kind: FromKind) -> FromItem {
        FromItem {
            kind,
            alias: None,
            columns: Vec::new(),
            lateral: false,
            sample: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CteDef {
    pub name: &'static str,
    pub columns: Vec<&'static str>,
    pub body: Stmt,
    pub recursive: bool,
    pub materialized: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct SetOpNode {
    pub op: &'static str,
    pub left: Stmt,
    pub right: Stmt,
    pub order: Vec<Node>,
}

impl Stmt {
    /// Output column names, where known.
    pub(crate) fn output_names(&self) -> Vec<Option<&'static str>> {
        match self {
            Stmt::Select(s) => s.items.iter().map(Item::name).collect(),
            Stmt::SetOp(s) => s.left.output_names(),
            Stmt::Values(rows) => (0..rows[0].len()).map(|i| Some(COLUMN_NAMES[i])).collect(),
            Stmt::Insert(s) => s.returning.iter().map(Item::name).collect(),
            Stmt::Update(s) => s.returning.iter().map(Item::name).collect(),
            Stmt::Delete(s) => s.returning.iter().map(Item::name).collect(),
            Stmt::Merge(s) => s.returning.iter().map(Item::name).collect(),
        }
    }

    /// Unique column names for use as a derived table or CTE.
    pub(crate) fn column_names(&self) -> Vec<&'static str> {
        let mut names: Vec<&'static str> = Vec::new();
        for (i, name) in self.output_names().into_iter().enumerate() {
            let name = name
                .filter(|n| !names.contains(n))
                .unwrap_or(COLUMN_NAMES[i]);
            names.push(name);
        }
        names
    }
}

/// Output column positions, used to order set operations.
const POSITIONS: [&str; 32] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17",
    "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32",
];

/// PostgreSQL's names for `VALUES` columns, also used for unnamed outputs.
const COLUMN_NAMES: [&str; 32] = [
    "column1", "column2", "column3", "column4", "column5", "column6", "column7", "column8",
    "column9", "column10", "column11", "column12", "column13", "column14", "column15", "column16",
    "column17", "column18", "column19", "column20", "column21", "column22", "column23", "column24",
    "column25", "column26", "column27", "column28", "column29", "column30", "column31", "column32",
];

// ---------------------------------------------------------------------------
// Scope checking

/// Scope requirement: `S` must be in scope within sources `F` or the
/// enclosing context (a correlated subquery).
pub struct Sub<S, F>(PhantomData<(S, F)>);
/// Scope requirement checked against the enclosing context only, skipping
/// the sources of the current query (a non-`LATERAL` derived table).
pub struct Outside<S>(PhantomData<S>);

pub struct Here;
pub struct Left<I>(PhantomData<I>);
pub struct Right<I>(PhantomData<I>);

/// Source tree `Self` contains source `Q` at position `I`.
pub trait Has<Q, I> {}
impl<Q> Has<Q, Here> for In<Q> {}
impl<Q, A: Has<Q, I>, B, I> Has<Q, Left<I>> for (A, B) {}
impl<Q, A, B: Has<Q, I>, I> Has<Q, Right<I>> for (A, B) {}

/// Every source referenced by `Self` is available in context `Ctx`; `I`
/// locates them and is inferred.
pub trait Scope<Ctx, I> {}
impl<Ctx> Scope<Ctx, ()> for () {}
impl<Ctx, A: Scope<Ctx, IA>, B: Scope<Ctx, IB>, IA, IB> Scope<Ctx, (IA, IB)> for (A, B) {}
impl<Ctx: Has<Q, I>, Q, I> Scope<Ctx, I> for In<Q> {}
impl<Ctx, S: Scope<(F, Ctx), I>, F, I> Scope<Ctx, I> for Sub<S, F> {}
impl<F, Ctx, S: Scope<Ctx, I>, I> Scope<(F, Ctx), I> for Outside<S> {}

/// Every source in the tree may be NULL-extended.
pub trait AllNullable {}
impl<Q: Qualifier<Null = Nullable>> AllNullable for In<Q> {}
impl<A: AllNullable, B: AllNullable> AllNullable for (A, B) {}

// ---------------------------------------------------------------------------
// Source identities

/// Source `Q` under an alias distinguished by `Tag`.
pub struct Alias<Q, Tag>(PhantomData<(Q, Tag)>);
/// Source `Q` on the NULL-extended side of an outer join.
pub struct Outer<Q>(PhantomData<Q>);
/// `old` row of `Q` in `RETURNING`.
pub struct Old<Q>(PhantomData<Q>);
/// `new` row of `Q` in `RETURNING`.
pub struct New<Q>(PhantomData<Q>);
/// `excluded` row of `Q` in `ON CONFLICT DO UPDATE`.
pub struct Excluded<Q>(PhantomData<Q>);
/// A derived table, function or CTE producing rows `R`.
pub struct Derived<R>(PhantomData<R>);
/// The output columns of a set operation, for its `ORDER BY`.
pub struct Output;

impl<Q: Qualifier, Tag: 'static> Qualifier for Alias<Q, Tag> {
    type Null = Q::Null;
}
impl<Q: 'static> Qualifier for Outer<Q> {
    type Null = Nullable;
}
impl<Q: 'static> Qualifier for Old<Q> {
    type Null = Nullable;
}
impl<Q: 'static> Qualifier for New<Q> {
    type Null = Nullable;
}
impl<Q: Qualifier> Qualifier for Excluded<Q> {
    type Null = Q::Null;
}
impl<R: 'static> Qualifier for Derived<R> {
    type Null = NotNull;
}
impl Qualifier for Output {
    type Null = NotNull;
}

// ---------------------------------------------------------------------------
// Sources

/// Something that can appear in `FROM`.
pub trait Source: Sized {
    /// Identity of the source for scope checking.
    type Id: Qualifier;
    /// Sources referenced by the source itself (`LATERAL` references).
    type Scope;
    /// The fields of `source.*`.
    type Row;

    fn into_from_item(self) -> FromItem;

    /// The name columns are qualified with.
    fn qualifier(&self) -> &'static str;

    /// `source.*`
    fn star(&self) -> Star<Self::Row, In<Self::Id>> {
        Star(Node::Column(self.qualifier(), "*"), PhantomData)
    }

    /// `SELECT projection FROM source`
    fn select<P: Projection>(
        self,
        projection: P,
    ) -> Select<In<Self::Id>, (Self::Scope, P::Scope), P::Row> {
        from_source(self).select(projection)
    }

    /// `SELECT source.* FROM source WHERE condition`
    fn filter<P: Predicate>(
        self,
        condition: P,
    ) -> Select<In<Self::Id>, (Self::Scope, P::Scope), Self::Row> {
        from_source(self).filter(condition)
    }

    /// `SELECT source.* FROM source ORDER BY keys`
    fn order_by<O: OrderBy>(
        self,
        keys: O,
    ) -> Select<In<Self::Id>, (Self::Scope, O::Scope), Self::Row> {
        from_source(self).order_by(keys)
    }

    /// `SELECT source.* FROM source GROUP BY keys`
    fn group_by<G: Exprs>(
        self,
        keys: G,
    ) -> Select<In<Self::Id>, (Self::Scope, G::Scope), Self::Row> {
        from_source(self).group_by(keys)
    }

    /// `FROM source JOIN other ON condition`
    fn join<T: Source, P: Predicate>(self, other: T, on: P) -> Joined<Self, T, P> {
        from_source(self).join(other, on)
    }

    /// `FROM source LEFT JOIN other ON condition`; `other` must be
    /// [`nullable`](Requalify::nullable).
    fn left_join<T: Source, P: Predicate>(self, other: T, on: P) -> Joined<Self, T, P>
    where
        T::Id: Qualifier<Null = Nullable>,
    {
        from_source(self).left_join(other, on)
    }

    /// `FROM source RIGHT JOIN other ON condition`; `source` must be
    /// [`nullable`](Requalify::nullable).
    fn right_join<T: Source, P: Predicate>(self, other: T, on: P) -> Joined<Self, T, P>
    where
        Self::Id: Qualifier<Null = Nullable>,
    {
        from_source(self).right_join(other, on)
    }

    /// `FROM source FULL JOIN other ON condition`; both must be
    /// [`nullable`](Requalify::nullable).
    fn full_join<T: Source, P: Predicate>(self, other: T, on: P) -> Joined<Self, T, P>
    where
        Self::Id: Qualifier<Null = Nullable>,
        T::Id: Qualifier<Null = Nullable>,
    {
        from_source(self).full_join(other, on)
    }

    /// `FROM source CROSS JOIN other`
    fn cross_join<T: Source>(
        self,
        other: T,
    ) -> Select<(In<Self::Id>, In<T::Id>), (Self::Scope, T::Scope), Self::Row> {
        from_source(self).cross_join(other)
    }

    /// `FROM source NATURAL JOIN other`
    fn natural_join<T: Source>(
        self,
        other: T,
    ) -> Select<(In<Self::Id>, In<T::Id>), (Self::Scope, T::Scope), Self::Row> {
        from_source(self).natural_join(other)
    }

    /// `FROM source LEFT JOIN other USING (columns)`; `other` must be
    /// [`nullable`](Requalify::nullable).
    fn left_join_using<T: Source, C: UsingColumns<T::Id>>(
        self,
        other: T,
        columns: C,
    ) -> Select<(In<Self::Id>, In<T::Id>), (Self::Scope, T::Scope), Self::Row>
    where
        T::Id: Qualifier<Null = Nullable>,
    {
        from_source(self).left_join_using(other, columns)
    }

    /// `source TABLESAMPLE BERNOULLI (percent)`
    fn tablesample_bernoulli<P: Arg<Float4>>(self, percent: P) -> Sampled<Self, P::Scope> {
        Sampled::new(self, "BERNOULLI", percent.into_arg())
    }
}

/// The query produced by joining `T` to `Source` `L` on `P`.
pub type Joined<L, T, P> = Select<
    (In<<L as Source>::Id>, In<<T as Source>::Id>),
    (
        (<L as Source>::Scope, <T as Source>::Scope),
        <P as Expression>::Scope,
    ),
    <L as Source>::Row,
>;

/// Sources that can be renamed, giving their columns a new identity.
pub trait Requalify: Source {
    type As<Q: Qualifier>: Source<Id = Q, Scope = Self::Scope>;

    /// The same source with identity `Q`, qualified by `name` if given.
    fn requalify<Q: Qualifier>(self, name: Option<&'static str>) -> Self::As<Q>;

    /// The source on the NULL-extended side of an outer join: its columns
    /// are nullable.
    fn nullable(self) -> Self::As<Outer<Self::Id>> {
        self.requalify(None)
    }
}

/// Things that can be given an alias, producing a source with a new
/// identity; use the [`alias!`](crate::alias) macro.
pub trait Aliasable {
    type As<Tag: 'static>;
    fn alias<Tag: 'static>(self, name: &'static str) -> Self::As<Tag>;
}

impl<T: Requalify> Aliasable for T {
    type As<Tag: 'static> = T::As<Alias<T::Id, Tag>>;
    fn alias<Tag: 'static>(self, name: &'static str) -> Self::As<Tag> {
        self.requalify(Some(name))
    }
}

/// A source other than a table: a derived table, `VALUES`, a set-returning
/// function or a CTE, with identity `Q`, row `R` and lateral references `S`.
pub struct Rel<Q, R, S = ()> {
    pub(crate) item: FromItem,
    pub(crate) qualifier: &'static str,
    pub(crate) names: Vec<&'static str>,
    _type: PhantomData<fn() -> (Q, R, S)>,
}

impl<Q, R, S> Clone for Rel<Q, R, S> {
    fn clone(&self) -> Self {
        Rel::new(self.item.clone(), self.qualifier, self.names.clone())
    }
}

impl<Q, R, S> Rel<Q, R, S> {
    pub(crate) fn new(item: FromItem, qualifier: &'static str, names: Vec<&'static str>) -> Self {
        Rel {
            item,
            qualifier,
            names,
            _type: PhantomData,
        }
    }

    fn cast<Q2, R2, S2>(self) -> Rel<Q2, R2, S2> {
        Rel::new(self.item, self.qualifier, self.names)
    }

    /// The columns of the source.
    pub fn columns(&self) -> R::Columns<Q>
    where
        R: Row,
    {
        R::columns(self.qualifier, &self.names)
    }

    /// Marks a CTE `MATERIALIZED`.
    pub fn materialized(self) -> Self {
        self.with_materialized(true)
    }

    /// Marks a CTE `NOT MATERIALIZED`.
    pub fn not_materialized(self) -> Self {
        self.with_materialized(false)
    }

    fn with_materialized(mut self, materialized: bool) -> Self {
        if let FromKind::Cte(def) = &mut self.item.kind {
            Rc::make_mut(def).materialized = Some(materialized);
        }
        self
    }
}

impl<Q: Qualifier, R, S> Source for Rel<Q, R, S> {
    type Id = Q;
    type Scope = S;
    type Row = R;

    fn into_from_item(self) -> FromItem {
        self.item
    }

    fn qualifier(&self) -> &'static str {
        self.qualifier
    }
}

impl<Q: Qualifier, R, S> Requalify for Rel<Q, R, S> {
    type As<Q2: Qualifier> = Rel<Q2, R, S>;

    fn requalify<Q2: Qualifier>(mut self, name: Option<&'static str>) -> Rel<Q2, R, S> {
        if let Some(name) = name {
            self.item.alias = Some(name);
            self.qualifier = name;
        }
        self.cast()
    }
}

/// `LATERAL` derived table: may reference sources joined before it.
pub fn lateral<Q, R, S>(derived: Rel<Q, R, Outside<S>>) -> Rel<Q, R, S> {
    let mut derived = derived.cast();
    derived.item.lateral = true;
    derived
}

/// A source with `TABLESAMPLE`.
pub struct Sampled<T, S> {
    source: T,
    sample: Vec<Node>,
    _scope: PhantomData<fn() -> S>,
}

impl<T: Source, S> Sampled<T, S> {
    fn new(source: T, method: &'static str, percent: Node) -> Self {
        let sample = vec![
            Node::Keyword("TABLESAMPLE"),
            Node::Keyword(method),
            Node::List(vec![percent]),
        ];
        Sampled {
            source,
            sample,
            _scope: PhantomData,
        }
    }

    /// `REPEATABLE (seed)`
    pub fn repeatable<P: Arg<Float8>>(mut self, seed: P) -> Sampled<T, (S, P::Scope)> {
        self.sample.extend([
            Node::Keyword("REPEATABLE"),
            Node::List(vec![seed.into_arg()]),
        ]);
        Sampled {
            source: self.source,
            sample: self.sample,
            _scope: PhantomData,
        }
    }
}

impl<T: Source, S> Source for Sampled<T, S> {
    type Id = T::Id;
    type Scope = (T::Scope, Outside<S>);
    type Row = T::Row;

    fn into_from_item(self) -> FromItem {
        let mut item = self.source.into_from_item();
        item.sample = Some(Node::Seq(self.sample));
        item
    }

    fn qualifier(&self) -> &'static str {
        self.source.qualifier()
    }
}

/// Gives a source or subquery an alias with a fresh identity, so that the
/// same table can be joined more than once: `alias!(users, "u")`.
#[macro_export]
macro_rules! alias {
    ($source:expr, $name:literal) => {{
        enum Tag {}
        $crate::Aliasable::alias::<Tag>($source, $name)
    }};
}

// ---------------------------------------------------------------------------
// Rows and projections

/// A result row: a tuple of [`Field`]s.
pub trait Row {
    /// Column handles for a source producing this row.
    type Columns<Q>;
    fn columns<Q>(qualifier: &'static str, names: &[&'static str]) -> Self::Columns<Q>;
}

/// Rows of the same column types; `Out` combines nullability.
pub trait Unify<Other> {
    type Out;
}

impl<R> Unify<Unreachable> for R {
    type Out = ();
}

/// Elements of a `SELECT` list.
pub trait SelectItem {
    type Field;
    type Scope;
    fn into_item(self) -> Item;
}

impl<E: Expression> SelectItem for E {
    type Field = <E::Null as Nullability>::Field<E::Sql>;
    type Scope = E::Scope;
    fn into_item(self) -> Item {
        Item {
            node: self.into_node(),
            alias: None,
        }
    }
}

/// An expression with an output name.
pub struct As<E>(pub(crate) E, pub(crate) &'static str);

impl<E: Expression> SelectItem for As<E> {
    type Field = <E::Null as Nullability>::Field<E::Sql>;
    type Scope = E::Scope;
    fn into_item(self) -> Item {
        Item {
            node: self.0.into_node(),
            alias: Some(self.1),
        }
    }
}

/// `source.*` in a `SELECT` list; its field is the source's row.
pub struct Star<R, S>(pub(crate) Node, PhantomData<fn() -> (R, S)>);

impl<R, S> Star<R, S> {
    pub(crate) fn new(node: Node) -> Self {
        Star(node, PhantomData)
    }
}

impl<R, S> SelectItem for Star<R, S> {
    type Field = R;
    type Scope = S;
    fn into_item(self) -> Item {
        Item {
            node: self.0,
            alias: None,
        }
    }
}

/// A `SELECT` list: one item or a tuple of items.
pub trait Projection {
    type Row;
    type Scope;
    fn push_items(self, items: &mut Vec<Item>);
}

impl<A: SelectItem> Projection for A {
    type Row = (A::Field,);
    type Scope = A::Scope;
    fn push_items(self, items: &mut Vec<Item>) {
        items.push(self.into_item());
    }
}

/// A list of expressions: one expression or a tuple of lists.
pub trait Exprs {
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

impl<E: Expression> Exprs for E {
    type Scope = E::Scope;
    fn push_nodes(self, nodes: &mut Vec<Node>) {
        nodes.push(self.into_node());
    }
}

impl Exprs for () {
    type Scope = ();
    fn push_nodes(self, _: &mut Vec<Node>) {}
}

/// `ORDER BY` keys: an expression, a [`Ordered`] key or a tuple of keys.
pub trait OrderBy {
    type Scope;
    fn push_keys(self, keys: &mut Vec<Node>);
}

impl<E: Expression> OrderBy for E {
    type Scope = E::Scope;
    fn push_keys(self, keys: &mut Vec<Node>) {
        keys.push(self.into_node());
    }
}

impl<S> OrderBy for Ordered<S> {
    type Scope = S;
    fn push_keys(self, keys: &mut Vec<Node>) {
        keys.push(self.node);
    }
}

/// Columns of source `Q` for `USING (...)`.
pub trait UsingColumns<Q> {
    fn names(self) -> Vec<&'static str>;
}

impl<T, N, Q> UsingColumns<Q> for Column<T, N, Q> {
    fn names(self) -> Vec<&'static str> {
        vec![self.name]
    }
}

/// `ROLLUP (...)`
pub fn rollup<E: Exprs>(keys: E) -> Expr<Bool, NotNull, E::Scope> {
    Expr::new(Node::call("ROLLUP", keys.into_nodes()))
}

/// `CUBE (...)`
pub fn cube<E: Exprs>(keys: E) -> Expr<Bool, NotNull, E::Scope> {
    Expr::new(Node::call("CUBE", keys.into_nodes()))
}

/// `GROUPING SETS (...)`, given a tuple of sets; each set is a tuple of
/// expressions, `()` being the empty set.
pub fn grouping_sets<G: GroupingSets>(sets: G) -> Expr<Bool, NotNull, G::Scope> {
    let mut nodes = Vec::new();
    sets.push_sets(&mut nodes);
    Expr::new(Node::call("GROUPING SETS", nodes))
}

/// A tuple of grouping sets.
pub trait GroupingSets {
    type Scope;
    fn push_sets(self, sets: &mut Vec<Node>);
}

macro_rules! tuples {
    ($(($($t:ident $i:tt),+))*) => {$(
        impl<$($t: SelectItem),+> Projection for ($($t,)+) {
            type Row = ($($t::Field,)+);
            type Scope = tuples!(@scope $($t::Scope),+);
            fn push_items(self, items: &mut Vec<Item>) {
                $(items.push(self.$i.into_item());)+
            }
        }

        impl<$($t: Field),+> Row for ($($t,)+) {
            type Columns<Q> = ($(Column<$t::Sql, $t::Null, Q>,)+);
            fn columns<Q>(qualifier: &'static str, names: &[&'static str]) -> Self::Columns<Q> {
                ($(Column::new(qualifier, names[$i]),)+)
            }
        }

        impl<$($t: Exprs),+> Exprs for ($($t,)+) {
            type Scope = tuples!(@scope $($t::Scope),+);
            fn push_nodes(self, nodes: &mut Vec<Node>) {
                $(self.$i.push_nodes(nodes);)+
            }
        }

        impl<$($t: Exprs),+> GroupingSets for ($($t,)+) {
            type Scope = tuples!(@scope $($t::Scope),+);
            fn push_sets(self, sets: &mut Vec<Node>) {
                $(sets.push(Node::List(self.$i.into_nodes()));)+
            }
        }

        impl<$($t: OrderBy),+> OrderBy for ($($t,)+) {
            type Scope = tuples!(@scope $($t::Scope),+);
            fn push_keys(self, keys: &mut Vec<Node>) {
                $(self.$i.push_keys(keys);)+
            }
        }

        impl<Q, $($t: UsingColumns<Q>),+> UsingColumns<Q> for ($($t,)+) {
            fn names(self) -> Vec<&'static str> {
                let mut names = Vec::new();
                $(names.extend(self.$i.names());)+
                names
            }
        }

        impl<$($t: Expression),+> ValuesRow for ($($t,)+) {
            type Row = ($(<$t::Null as Nullability>::Field<$t::Sql>,)+);
            type Scope = tuples!(@scope $($t::Scope),+);
            fn into_nodes(self) -> Vec<Node> {
                vec![$(self.$i.into_node()),+]
            }
        }
    )*};
    (@scope $a:ty) => { $a };
    (@scope $a:ty, $($rest:ty),+) => { ($a, tuples!(@scope $($rest),+)) };
}

/// Implements a trait for tuples of 1 to 32 elements.
macro_rules! for_tuples {
    ($mac:ident) => {
        $mac! {
            (T0 0)
            (T0 0, T1 1)
            (T0 0, T1 1, T2 2)
            (T0 0, T1 1, T2 2, T3 3)
            (T0 0, T1 1, T2 2, T3 3, T4 4)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22, T23 23)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22, T23 23, T24 24)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22, T23 23, T24 24, T25 25)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22, T23 23, T24 24, T25 25, T26 26)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22, T23 23, T24 24, T25 25, T26 26, T27 27)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22, T23 23, T24 24, T25 25, T26 26, T27 27, T28 28)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22, T23 23, T24 24, T25 25, T26 26, T27 27, T28 28, T29 29)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22, T23 23, T24 24, T25 25, T26 26, T27 27, T28 28, T29 29, T30 30)
            (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14, T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22, T23 23, T24 24, T25 25, T26 26, T27 27, T28 28, T29 29, T30 30, T31 31)
        }
    };
}

pub(crate) use for_tuples;

for_tuples!(tuples);

/// Elementwise row compatibility, for tuples of up to 12 fields.
macro_rules! unify {
    ($(($($t:ident $u:ident),+))*) => {$(
        impl<$($t: Field,)+ $($u: Field),+> Unify<($($u,)+)> for ($($t,)+)
        where
            $($t::Sql: SameType<$u::Sql>,)+
        {
            type Out = ($(<<$t::Null as Nullability>::Or<$u::Null> as Nullability>::Field<$t::Sql>,)+);
        }
    )*};
}

unify! {
    (A A2)
    (A A2, B B2)
    (A A2, B B2, C C2)
    (A A2, B B2, C C2, D D2)
    (A A2, B B2, C C2, D D2, E E2)
    (A A2, B B2, C C2, D D2, E E2, F F2)
    (A A2, B B2, C C2, D D2, E E2, F F2, G G2)
    (A A2, B B2, C C2, D D2, E E2, F F2, G G2, H H2)
    (A A2, B B2, C C2, D D2, E E2, F F2, G G2, H H2, I I2)
    (A A2, B B2, C C2, D D2, E E2, F F2, G G2, H H2, I I2, J J2)
    (A A2, B B2, C C2, D D2, E E2, F F2, G G2, H H2, I I2, J J2, K K2)
    (A A2, B B2, C C2, D D2, E E2, F F2, G G2, H H2, I I2, J J2, K K2, L L2)
}

// ---------------------------------------------------------------------------
// Statements

/// A complete statement that compiles to SQL.
pub trait Statement: Sized {
    /// The result row.
    type Row;
    /// Sources referenced from enclosing queries.
    type Scope;

    fn into_stmt(self) -> Stmt;

    /// Renders the statement, after checking that it references no source
    /// outside its scope.
    fn compile<I>(self) -> Compiled<Self::Row>
    where
        Self::Scope: Scope<(), I>,
    {
        crate::render::compile(self.into_stmt())
    }
}

/// Queries that combine with set operations: `SELECT`, `VALUES` and set
/// operations themselves.
pub trait SetOperand: Statement {
    /// `self UNION other`
    fn union<Q: SetOperand>(self, other: Q) -> Combined<Self, Q>
    where
        Self::Row: Unify<Q::Row>,
    {
        SetOp::new("UNION", self, other)
    }

    /// `self UNION ALL other`
    fn union_all<Q: SetOperand>(self, other: Q) -> Combined<Self, Q>
    where
        Self::Row: Unify<Q::Row>,
    {
        SetOp::new("UNION ALL", self, other)
    }

    /// `self INTERSECT other`
    fn intersect<Q: SetOperand>(self, other: Q) -> Combined<Self, Q>
    where
        Self::Row: Unify<Q::Row>,
    {
        SetOp::new("INTERSECT", self, other)
    }

    /// `self EXCEPT other`
    fn except<Q: SetOperand>(self, other: Q) -> Combined<Self, Q>
    where
        Self::Row: Unify<Q::Row>,
    {
        SetOp::new("EXCEPT", self, other)
    }
}

/// The result of a set operation.
pub type Combined<L, R> = SetOp<
    (<L as Statement>::Scope, <R as Statement>::Scope),
    <<L as Statement>::Row as Unify<<R as Statement>::Row>>::Out,
>;

/// A query used as a derived table.
macro_rules! derived {
    ($ty:ident<$($p:ident),*>) => {
        impl<$($p,)* R> Aliasable for $ty<$($p,)* R>
        where
            Self: Statement,
        {
            type As<Tag: 'static> = Rel<Alias<Derived<<Self as Statement>::Row>, Tag>, <Self as Statement>::Row, Outside<<Self as Statement>::Scope>>;

            fn alias<Tag: 'static>(self, name: &'static str) -> Self::As<Tag> {
                let stmt = self.into_stmt();
                let names = stmt.column_names();
                let mut item = FromItem::new(FromKind::Query(Box::new(stmt)));
                item.alias = Some(name);
                item.columns = names.clone();
                Rel::new(item, name, names)
            }
        }

        /// A single-column query as a scalar subquery.
        impl<$($p,)* X: Field> Expression for $ty<$($p,)* (X,)>
        where
            Self: Statement,
        {
            type Sql = X::Sql;
            type Null = Nullable;
            type Scope = <Self as Statement>::Scope;
            fn into_node(self) -> Node {
                Node::Subquery(Box::new(self.into_stmt()))
            }
        }

        impl<$($p,)* X: Field> ExpressionMethods for $ty<$($p,)* (X,)> where Self: Statement {}

        /// A single-column query as the right-hand side of `IN`.
        impl<T, $($p,)* X: Field> InList<T> for $ty<$($p,)* (X,)>
        where
            Self: Statement,
            X::Sql: Coerce<T>,
        {
            type Null = Nullable;
            type Scope = <Self as Statement>::Scope;
            fn into_node(self) -> Node {
                Node::Subquery(Box::new(self.into_stmt()))
            }
        }
    };
}

/// `EXISTS (query)`
pub fn exists<Q: Statement>(query: Q) -> Expr<Bool, NotNull, Q::Scope> {
    Expr::new(Node::Prefix(
        "EXISTS",
        Box::new(Node::Subquery(Box::new(query.into_stmt()))),
    ))
}

/// `ANY (...)` as the right operand of an operator.
pub fn any<A: Quantified>(values: A) -> Quantifier<A::Sql, A::Scope> {
    Quantifier::new("ANY", values.into_node())
}

/// `ALL (...)` as the right operand of an operator.
pub fn all<A: Quantified>(values: A) -> Quantifier<A::Sql, A::Scope> {
    Quantifier::new("ALL", values.into_node())
}

/// An array or a single-column subquery, for `ANY`/`ALL`.
pub trait Quantified {
    type Sql: SqlType;
    type Scope;
    fn into_node(self) -> Node;
}

impl<F, S, X: Field> Quantified for Select<F, S, (X,)> {
    type Sql = X::Sql;
    type Scope = Sub<S, F>;
    fn into_node(self) -> Node {
        Node::Subquery(Box::new(self.into_stmt()))
    }
}

impl<T: SqlType, V: Bind<Sql = Array<T>>> Quantified for V {
    type Sql = T;
    type Scope = ();
    fn into_node(self) -> Node {
        Node::List(vec![Node::param(&self)])
    }
}

/// `ANY (...)` or `ALL (...)`.
pub struct Quantifier<T, S>(Node, PhantomData<fn() -> (T, S)>);

impl<T, S> Quantifier<T, S> {
    fn new(keyword: &'static str, values: Node) -> Self {
        Quantifier(Node::Seq(vec![Node::Keyword(keyword), values]), PhantomData)
    }
}

impl<T: SqlType, S> Operand for Quantifier<T, S> {
    type Sql = T;
    type Null = Nullable;
    type Scope = S;
    fn into_operand(self) -> Node {
        self.0
    }
}

// ---------------------------------------------------------------------------
// SELECT

/// A `SELECT` with sources `F`, referenced sources `S` and result row `R`.
pub struct Select<F, S, R> {
    node: SelectNode,
    _type: PhantomData<fn() -> (F, S, R)>,
}

/// `SELECT projection` without `FROM`.
pub fn select<P: Projection>(projection: P) -> Select<(), P::Scope, P::Row> {
    let mut node = SelectNode::default();
    projection.push_items(&mut node.items);
    Select {
        node,
        _type: PhantomData,
    }
}

/// `SELECT source.* FROM source`, the start of a query.
pub(crate) fn from_source<T: Source>(source: T) -> Select<In<T::Id>, T::Scope, T::Row> {
    let star = Item {
        node: Node::Column(source.qualifier(), "*"),
        alias: None,
    };
    let join = Join {
        kind: "",
        item: source.into_from_item(),
        condition: None,
    };
    Select {
        node: SelectNode {
            items: vec![star],
            from: vec![join],
            ..SelectNode::default()
        },
        _type: PhantomData,
    }
}

impl<F, S, R> Select<F, S, R> {
    fn retype<F2, S2, R2>(self) -> Select<F2, S2, R2> {
        Select {
            node: self.node,
            _type: PhantomData,
        }
    }

    fn add_join<F2, S2>(
        mut self,
        kind: &'static str,
        item: FromItem,
        condition: Option<Node>,
    ) -> Select<F2, S2, R> {
        self.node.from.push(Join {
            kind,
            item,
            condition,
        });
        self.retype()
    }

    /// Sets the `SELECT` list.
    pub fn select<P: Projection>(mut self, projection: P) -> Select<F, (S, P::Scope), P::Row> {
        self.node.items.clear();
        projection.push_items(&mut self.node.items);
        self.retype()
    }

    /// Adds a `WHERE` condition, combined with `AND`.
    pub fn filter<P: Predicate>(mut self, condition: P) -> Select<F, (S, P::Scope), R> {
        self.node.filter = Some(and(self.node.filter.take(), condition.into_node()));
        self.retype()
    }

    /// `JOIN other ON condition`
    pub fn join<T: Source, P: Predicate>(
        self,
        other: T,
        on: P,
    ) -> Select<(F, In<T::Id>), ((S, T::Scope), P::Scope), R> {
        self.add_join("JOIN", other.into_from_item(), Some(on_clause(on)))
    }

    /// `LEFT JOIN other ON condition`; `other` must be
    /// [`nullable`](Requalify::nullable).
    pub fn left_join<T: Source, P: Predicate>(
        self,
        other: T,
        on: P,
    ) -> Select<(F, In<T::Id>), ((S, T::Scope), P::Scope), R>
    where
        T::Id: Qualifier<Null = Nullable>,
    {
        self.add_join("LEFT JOIN", other.into_from_item(), Some(on_clause(on)))
    }

    /// `RIGHT JOIN other ON condition`; the sources joined so far must be
    /// [`nullable`](Requalify::nullable).
    pub fn right_join<T: Source, P: Predicate>(
        self,
        other: T,
        on: P,
    ) -> Select<(F, In<T::Id>), ((S, T::Scope), P::Scope), R>
    where
        F: AllNullable,
    {
        self.add_join("RIGHT JOIN", other.into_from_item(), Some(on_clause(on)))
    }

    /// `FULL JOIN other ON condition`; all sources must be
    /// [`nullable`](Requalify::nullable).
    pub fn full_join<T: Source, P: Predicate>(
        self,
        other: T,
        on: P,
    ) -> Select<(F, In<T::Id>), ((S, T::Scope), P::Scope), R>
    where
        F: AllNullable,
        T::Id: Qualifier<Null = Nullable>,
    {
        self.add_join("FULL JOIN", other.into_from_item(), Some(on_clause(on)))
    }

    /// `CROSS JOIN other`
    pub fn cross_join<T: Source>(self, other: T) -> Select<(F, In<T::Id>), (S, T::Scope), R> {
        self.add_join("CROSS JOIN", other.into_from_item(), None)
    }

    /// `NATURAL JOIN other`
    pub fn natural_join<T: Source>(self, other: T) -> Select<(F, In<T::Id>), (S, T::Scope), R> {
        self.add_join("NATURAL JOIN", other.into_from_item(), None)
    }

    /// `JOIN other USING (columns)`, naming columns of `other`.
    pub fn join_using<T: Source, C: UsingColumns<T::Id>>(
        self,
        other: T,
        columns: C,
    ) -> Select<(F, In<T::Id>), (S, T::Scope), R> {
        self.add_join("JOIN", other.into_from_item(), Some(using_clause(columns)))
    }

    /// `LEFT JOIN other USING (columns)`, naming columns of `other`.
    pub fn left_join_using<T: Source, C: UsingColumns<T::Id>>(
        self,
        other: T,
        columns: C,
    ) -> Select<(F, In<T::Id>), (S, T::Scope), R>
    where
        T::Id: Qualifier<Null = Nullable>,
    {
        self.add_join(
            "LEFT JOIN",
            other.into_from_item(),
            Some(using_clause(columns)),
        )
    }

    /// `GROUP BY keys`
    pub fn group_by<G: Exprs>(mut self, keys: G) -> Select<F, (S, G::Scope), R> {
        keys.push_nodes(&mut self.node.group);
        self.retype()
    }

    /// Adds a `HAVING` condition, combined with `AND`.
    pub fn having<P: Predicate>(mut self, condition: P) -> Select<F, (S, P::Scope), R> {
        self.node.having = Some(and(self.node.having.take(), condition.into_node()));
        self.retype()
    }

    /// `WINDOW name AS (...)`
    pub fn window<WS>(
        mut self,
        window: &crate::functions::NamedWindow<WS>,
    ) -> Select<F, (S, WS), R> {
        self.node.windows.push((window.name, window.spec.clone()));
        self.retype()
    }

    /// Appends `ORDER BY` keys.
    pub fn order_by<O: OrderBy>(mut self, keys: O) -> Select<F, (S, O::Scope), R> {
        keys.push_keys(&mut self.node.order);
        self.retype()
    }

    /// `SELECT DISTINCT`
    pub fn distinct(mut self) -> Self {
        self.node.distinct = Some(Vec::new());
        self
    }

    /// `SELECT DISTINCT ON (keys)`
    pub fn distinct_on<E: Exprs>(mut self, keys: E) -> Select<F, (S, E::Scope), R> {
        self.node.distinct = Some(keys.into_nodes());
        self.retype()
    }

    /// `LIMIT count`
    pub fn limit<N: Arg<Int8>>(mut self, count: N) -> Select<F, (S, Outside<N::Scope>), R> {
        self.node.limit = Some(count.into_arg());
        self.retype()
    }

    /// `OFFSET start`
    pub fn offset<N: Arg<Int8>>(mut self, start: N) -> Select<F, (S, Outside<N::Scope>), R> {
        self.node.offset = Some(start.into_arg());
        self.retype()
    }

    /// `FETCH FIRST count ROWS ONLY`
    pub fn fetch_first<N: Arg<Int8>>(mut self, count: N) -> Select<F, (S, Outside<N::Scope>), R> {
        self.node.fetch = Some((count.into_arg(), false));
        self.retype()
    }

    /// `FETCH FIRST count ROWS WITH TIES`
    pub fn fetch_first_with_ties<N: Arg<Int8>>(
        mut self,
        count: N,
    ) -> Select<F, (S, Outside<N::Scope>), R> {
        self.node.fetch = Some((count.into_arg(), true));
        self.retype()
    }

    fn lock(mut self, strength: &'static str) -> Self {
        self.node.locks.push(vec![Node::Keyword(strength)]);
        self
    }

    fn lock_option(mut self, node: Node) -> Self {
        if let Some(lock) = self.node.locks.last_mut() {
            lock.push(node);
        }
        self
    }

    /// `FOR UPDATE`
    pub fn for_update(self) -> Self {
        self.lock("FOR UPDATE")
    }

    /// `FOR NO KEY UPDATE`
    pub fn for_no_key_update(self) -> Self {
        self.lock("FOR NO KEY UPDATE")
    }

    /// `FOR SHARE`
    pub fn for_share(self) -> Self {
        self.lock("FOR SHARE")
    }

    /// `FOR KEY SHARE`
    pub fn for_key_share(self) -> Self {
        self.lock("FOR KEY SHARE")
    }

    /// Restricts the last locking clause: `OF source`.
    pub fn of<T: Source>(self, source: &T) -> Select<F, (S, In<T::Id>), R> {
        let qualifier = source.qualifier();
        self.lock_option(Node::Seq(vec![Node::Keyword("OF"), Node::Ident(qualifier)]))
            .retype()
    }

    /// `NOWAIT` for the last locking clause.
    pub fn nowait(self) -> Self {
        self.lock_option(Node::Keyword("NOWAIT"))
    }

    /// `SKIP LOCKED` for the last locking clause.
    pub fn skip_locked(self) -> Self {
        self.lock_option(Node::Keyword("SKIP LOCKED"))
    }
}

fn and(left: Option<Node>, right: Node) -> Node {
    match left {
        Some(left) => Node::binary("AND", left, right),
        None => right,
    }
}

fn on_clause<P: Expression>(condition: P) -> Node {
    Node::Seq(vec![Node::Keyword("ON"), condition.into_node()])
}

fn using_clause<Q, C: UsingColumns<Q>>(columns: C) -> Node {
    let names = columns.names().into_iter().map(Node::Ident).collect();
    Node::Seq(vec![Node::Keyword("USING"), Node::List(names)])
}

impl<F, S, R> Statement for Select<F, S, R> {
    type Row = R;
    type Scope = Sub<S, F>;
    fn into_stmt(self) -> Stmt {
        Stmt::Select(Box::new(self.node))
    }
}

impl<F, S, R> SetOperand for Select<F, S, R> {}

derived!(Select<F, S>);

// ---------------------------------------------------------------------------
// Set operations

/// `UNION`, `INTERSECT` or `EXCEPT` of two queries.
pub struct SetOp<S, R> {
    node: SetOpNode,
    _type: PhantomData<fn() -> (S, R)>,
}

impl<S, R> SetOp<S, R> {
    fn new<L: Statement, Q: Statement>(op: &'static str, left: L, right: Q) -> Self {
        let node = SetOpNode {
            op,
            left: left.into_stmt(),
            right: right.into_stmt(),
            order: Vec::new(),
        };
        SetOp {
            node,
            _type: PhantomData,
        }
    }

    /// `ORDER BY`, in terms of the output columns passed to `keys`.
    pub fn order_by<O: OrderBy, I>(mut self, keys: impl FnOnce(R::Columns<Output>) -> O) -> Self
    where
        R: Row,
        O::Scope: Scope<(In<Output>, ()), I>,
    {
        keys(R::columns("", &POSITIONS)).push_keys(&mut self.node.order);
        self
    }
}

impl<S, R> Statement for SetOp<S, R> {
    type Row = R;
    type Scope = S;
    fn into_stmt(self) -> Stmt {
        Stmt::SetOp(Box::new(self.node))
    }
}

impl<S, R> SetOperand for SetOp<S, R> {}

derived!(SetOp<S>);

// ---------------------------------------------------------------------------
// VALUES

/// A row of `VALUES`: a tuple of expressions.
pub trait ValuesRow {
    type Row;
    type Scope;
    fn into_nodes(self) -> Vec<Node>;
}

/// `VALUES (...), (...)`
pub struct Values<S, R> {
    rows: Vec<Vec<Node>>,
    _type: PhantomData<fn() -> (R, S)>,
}

/// `VALUES` from rows of equal type.
pub fn values<V: ValuesRow>(rows: impl IntoIterator<Item = V>) -> Values<V::Scope, V::Row> {
    Values {
        rows: rows.into_iter().map(ValuesRow::into_nodes).collect(),
        _type: PhantomData,
    }
}

impl<S, R> Statement for Values<S, R> {
    type Row = R;
    type Scope = S;
    fn into_stmt(self) -> Stmt {
        Stmt::Values(self.rows)
    }
}

impl<S, R> SetOperand for Values<S, R> {}

derived!(Values<S>);

// ---------------------------------------------------------------------------
// WITH

/// A CTE defined by `query`; referencing it in a statement adds it to that
/// statement's `WITH` clause.
pub fn cte<Q: Statement, I>(name: &'static str, query: Q) -> Rel<Derived<Q::Row>, Q::Row>
where
    Q::Scope: Scope<(), I>,
{
    let body = query.into_stmt();
    let columns = body.column_names();
    let def = CteDef {
        name,
        columns: columns.clone(),
        body,
        recursive: false,
        materialized: None,
    };
    Rel::new(FromItem::new(FromKind::Cte(Rc::new(def))), name, columns)
}

/// A recursive CTE: `base UNION ALL step(self)`.
pub fn recursive_cte<B, Q, I, J>(
    name: &'static str,
    base: B,
    step: impl FnOnce(Rel<Derived<B::Row>, B::Row>) -> Q,
) -> Rel<Derived<B::Row>, B::Row>
where
    B: SetOperand,
    B::Scope: Scope<(), I>,
    Q: SetOperand,
    Q::Scope: Scope<(), J>,
    B::Row: Unify<Q::Row>,
{
    let base = base.into_stmt();
    let columns = base.column_names();
    let this = Rel::new(
        FromItem::new(FromKind::CteSelf(name)),
        name,
        columns.clone(),
    );
    let body = Stmt::SetOp(Box::new(SetOpNode {
        op: "UNION ALL",
        left: base,
        right: step(this).into_stmt(),
        order: Vec::new(),
    }));
    let def = CteDef {
        name,
        columns: columns.clone(),
        body,
        recursive: true,
        materialized: None,
    };
    Rel::new(FromItem::new(FromKind::Cte(Rc::new(def))), name, columns)
}

rust_operators!(Select<F, S, R>, SetOp<S, R>, Values<S, R>);
