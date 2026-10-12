//! Support for the typed schema generated from the PostgreSQL catalog.
//!
//! A table is a struct with one field per column, generic over a `Mode`:
//!
//! - `Users<Unbound>` is the table itself, the constant `users`, which names
//!   the table in queries; its fields are `NotInScope` and not expressions;
//! - `Users<Bound>` is the table as a FROM item of a query, with columns;
//! - `Users<Nulled>` is the same on the nullable side of an outer join.

use crate::Row;
use crate::ast::{self, Node, Rel};
use crate::dml::{Delete, Insert, Insertable, Merge, Update, merge};
use crate::expr::Assignments;
use crate::expr::{Base, Column, Expr, Operand};
use crate::query::{
    FromItem, Grouping, OrderBy, Projection, Select, Source, Sources, from, next_id,
};
use crate::types::*;
use std::marker::PhantomData;

/// Identifies a FROM item of a query.
#[derive(Clone, Copy)]
pub struct Alias(pub(crate) Rel);

impl Alias {
    pub fn column<M: Mode, T: PgType>(self, name: &'static str) -> M::Column<T> {
        M::column(self, name)
    }
}

/// How the columns of a table are exposed.
pub trait Mode: Copy + 'static {
    type Nullable: Mode;
    type Type<T: PgType>: PgType;
    type Column<T: PgType>: Copy;
    fn column<T: PgType>(alias: Alias, name: &'static str) -> Self::Column<T>;
    fn nullable<T: PgType>(column: Self::Column<T>) -> <Self::Nullable as Mode>::Column<T>;
    fn alias<T: PgType>(column: &Self::Column<T>) -> Alias;
}

/// Columns of a FROM item.
#[derive(Clone, Copy)]
pub struct Bound;

/// Columns on the nullable side of an outer join.
#[derive(Clone, Copy)]
pub struct Nulled;

/// The table itself, outside any query.
#[derive(Clone, Copy)]
pub struct Unbound;

/// A column of a table that is not a FROM item of the query: use the
/// columns of the source the query hands to its closures instead.
#[derive(Clone, Copy)]
pub struct NotInScope;

impl Mode for Bound {
    type Nullable = Nulled;
    type Type<T: PgType> = T;
    type Column<T: PgType> = Column<T>;
    fn column<T: PgType>(alias: Alias, name: &'static str) -> Column<T> {
        Column::new(alias.0, name)
    }
    fn nullable<T: PgType>(column: Column<T>) -> Column<Nullable<T::Base>> {
        column.nullable()
    }
    fn alias<T: PgType>(column: &Column<T>) -> Alias {
        Alias(column.rel())
    }
}

impl Mode for Nulled {
    type Nullable = Nulled;
    type Type<T: PgType> = Nullable<T::Base>;
    type Column<T: PgType> = Column<Nullable<T::Base>>;
    fn column<T: PgType>(alias: Alias, name: &'static str) -> Self::Column<T> {
        Column::new(alias.0, name)
    }
    fn nullable<T: PgType>(column: Self::Column<T>) -> Self::Column<T> {
        column
    }
    fn alias<T: PgType>(column: &Self::Column<T>) -> Alias {
        Alias(column.rel())
    }
}

impl Mode for Unbound {
    type Nullable = Unbound;
    type Type<T: PgType> = T;
    type Column<T: PgType> = NotInScope;
    fn column<T: PgType>(_: Alias, _: &'static str) -> NotInScope {
        NotInScope
    }
    fn nullable<T: PgType>(_: NotInScope) -> NotInScope {
        NotInScope
    }
    fn alias<T: PgType>(_: &NotInScope) -> Alias {
        unreachable!("a table outside a query has no alias")
    }
}

/// A table, view or other relation as a FROM item of a query.
pub trait Scoped: Source {
    /// The row of `source.*`.
    type Row;
    fn scoped(alias: Alias) -> Self;
    fn alias(&self) -> Alias;

    /// `source.*`, all columns.
    fn star(&self) -> Star<Self::Row> {
        Star(self.alias().0, PhantomData)
    }
}

/// The projection `source.*`.
pub struct Star<R>(pub(crate) Rel, PhantomData<R>);

/// A table or view, the constant naming it in queries; its methods start a
/// query with the table as the first FROM item.
pub trait Table: Copy + 'static {
    type Source: Scoped;
    /// The builder of the values of an inserted row.
    type Insert: Default;
    const SCHEMA: &'static str;
    const NAME: &'static str;

    fn select<P: Projection>(
        self,
        projection: impl FnOnce(Self::Source) -> P,
    ) -> Select<(Self::Source,), P::Row> {
        from(self).select(projection)
    }

    fn filter<E: Operand>(
        self,
        condition: impl FnOnce(Self::Source) -> E,
    ) -> Select<(Self::Source,)>
    where
        Base<E>: Boolean,
    {
        from(self).filter(condition)
    }

    fn order_by<O: OrderBy>(
        self,
        order: impl FnOnce(Self::Source) -> O,
    ) -> Select<(Self::Source,)> {
        from(self).order_by(order)
    }

    fn group_by<G: Grouping>(
        self,
        keys: impl FnOnce(Self::Source) -> G,
    ) -> Select<G::Scope<(Self::Source,)>> {
        from(self).group_by(keys)
    }

    fn join<F: FromItem, E: Operand>(
        self,
        item: F,
        on: impl FnOnce((Self::Source, F::Source)) -> E,
    ) -> Select<(Self::Source, F::Source)>
    where
        Base<E>: Boolean,
    {
        from(self).join(item, on)
    }

    fn left_join<F: FromItem, E: Operand>(
        self,
        item: F,
        on: impl FnOnce((Self::Source, <F::Source as Source>::Nullable)) -> E,
    ) -> Select<(Self::Source, <F::Source as Source>::Nullable)>
    where
        Base<E>: Boolean,
    {
        from(self).left_join(item, on)
    }

    fn right_join<F: FromItem, E: Operand>(
        self,
        item: F,
        on: impl FnOnce((<Self::Source as Source>::Nullable, F::Source)) -> E,
    ) -> Select<(<Self::Source as Source>::Nullable, F::Source)>
    where
        Base<E>: Boolean,
    {
        from(self).right_join(item, on)
    }

    fn full_join<F: FromItem, E: Operand>(
        self,
        item: F,
        on: impl FnOnce(
            (
                <Self::Source as Source>::Nullable,
                <F::Source as Source>::Nullable,
            ),
        ) -> E,
    ) -> Select<(
        <Self::Source as Source>::Nullable,
        <F::Source as Source>::Nullable,
    )>
    where
        Base<E>: Boolean,
    {
        from(self).full_join(item, on)
    }

    fn cross_join<F: FromItem>(self, item: F) -> Select<(Self::Source, F::Source)> {
        from(self).cross_join(item)
    }

    fn join_lateral<F: FromItem, E: Operand>(
        self,
        item: impl FnOnce(Self::Source) -> F,
        on: impl FnOnce((Self::Source, F::Source)) -> E,
    ) -> Select<(Self::Source, F::Source)>
    where
        Base<E>: Boolean,
    {
        from(self).join_lateral(item, on)
    }

    fn left_join_lateral<F: FromItem, E: Operand>(
        self,
        item: impl FnOnce(Self::Source) -> F,
        on: impl FnOnce((Self::Source, <F::Source as Source>::Nullable)) -> E,
    ) -> Select<(Self::Source, <F::Source as Source>::Nullable)>
    where
        Base<E>: Boolean,
    {
        from(self).left_join_lateral(item, on)
    }

    fn cross_join_lateral<F: FromItem>(
        self,
        item: impl FnOnce(Self::Source) -> F,
    ) -> Select<(Self::Source, F::Source)> {
        from(self).cross_join_lateral(item)
    }

    /// `INSERT INTO table (...) VALUES (...)`, the row built by setting
    /// columns, every required column included.
    fn insert<I: Insertable<Table = Self>>(
        self,
        row: impl FnOnce(Self::Insert) -> I,
    ) -> Insert<Self> {
        Insert::new(self, row(Self::Insert::default()))
    }

    /// `INSERT INTO table DEFAULT VALUES`.
    fn insert_defaults(self) -> Insert<Self>
    where
        Self::Insert: Insertable<Table = Self>,
    {
        Insert::defaults(self)
    }

    /// `INSERT INTO table (...) SELECT ...`, the row built from the sources
    /// of the query.
    fn insert_from<S: Sources, I: Insertable<Table = Self>>(
        self,
        query: Select<S>,
        row: impl FnOnce(Self::Insert, S::View) -> I,
    ) -> Insert<Self> {
        Insert::select(self, query, row)
    }

    /// `UPDATE table SET ...`, of all rows.
    fn update<A: Assignments>(
        self,
        set: impl FnOnce(Self::Source) -> A,
    ) -> Update<(Self::Source,)> {
        from(self).update(set)
    }

    /// `DELETE FROM table`, all rows.
    fn delete(self) -> Delete<(Self::Source,)> {
        from(self).delete()
    }

    /// `MERGE INTO table USING source ON condition`.
    fn merge<F: FromItem, E: Operand>(
        self,
        source: F,
        on: impl FnOnce((Self::Source, F::Source)) -> E,
    ) -> Merge<Self, F::Source>
    where
        Base<E>: Boolean,
    {
        merge(self, source, on)
    }

    /// `table TABLESAMPLE BERNOULLI (percent)`.
    fn tablesample_bernoulli<P: Operand>(self, percent: P) -> Sample<Self>
    where
        Base<P>: Coerce<Float4>,
    {
        Sample::new(self, "BERNOULLI", percent.into_node())
    }

    /// `table TABLESAMPLE SYSTEM (percent)`.
    fn tablesample_system<P: Operand>(self, percent: P) -> Sample<Self>
    where
        Base<P>: Coerce<Float4>,
    {
        Sample::new(self, "SYSTEM", percent.into_node())
    }
}

/// A table with a TABLESAMPLE clause.
pub struct Sample<T> {
    table: T,
    sample: ast::Sample,
}

impl<T: Table> Sample<T> {
    fn new(table: T, method: &'static str, percent: Node) -> Sample<T> {
        Sample {
            table,
            sample: ast::Sample {
                method,
                args: vec![percent],
                repeatable: None,
            },
        }
    }

    /// `REPEATABLE (seed)`.
    pub fn repeatable<S: Operand>(mut self, seed: S) -> Self
    where
        Base<S>: Coerce<Float8>,
    {
        self.sample.repeatable = Some(seed.into_node());
        self
    }
}

impl<T: Table> FromItem for Sample<T> {
    type Source = T::Source;
    fn into_from(self) -> (ast::From, T::Source) {
        let (mut from, source) = self.table.into_from();
        if let ast::Source::Table { sample, .. } = &mut from.source {
            *sample = Some(self.sample);
        }
        (from, source)
    }
}

impl<T: Table> FromItem for T {
    type Source = T::Source;
    fn into_from(self) -> (ast::From, T::Source) {
        let id = next_id();
        let from = ast::From {
            source: ast::Source::Table {
                schema: T::SCHEMA,
                name: T::NAME,
                sample: None,
            },
            rel: id,
            columns: 0,
        };
        (from, T::Source::scoped(Alias(Rel::Source(id))))
    }
}

/// A named constraint of the table `T`, for ON CONFLICT ON CONSTRAINT.
pub struct Constraint<T> {
    pub(crate) name: &'static str,
    table: PhantomData<T>,
}

impl<T> Constraint<T> {
    pub const fn new(name: &'static str) -> Constraint<T> {
        Constraint {
            name,
            table: PhantomData,
        }
    }
}

/// A composite type with its fields, all nullable.
pub trait Fields: Composite {
    type Row: Row;
}

/// The field `name` of a composite value.
pub fn field<T: PgType<Null = NotNull>, E: Operand>(
    value: E,
    name: &'static str,
) -> Expr<Nullable<T>>
where
    Base<E>: Composite,
{
    Expr::new(Node::Field(Box::new(value.into_node()), name))
}

/// `(value).*`, all fields of a composite value.
pub fn expand<E: Operand>(value: E) -> Expand<<Base<E> as Fields>::Row>
where
    Base<E>: Fields,
{
    Expand(Node::FieldStar(Box::new(value.into_node())), PhantomData)
}

/// The projection `(value).*`.
pub struct Expand<R>(pub(crate) Node, PhantomData<R>);

/// `ROW(fields...)::T`, a composite value.
pub fn composite<T: Composite>(fields: Vec<Node>) -> Expr<T> {
    Expr::new(Node::Cast(Box::new(Node::Row(fields)), T::name()))
}

impl<T: Enum + Label> Operand for T {
    type Type = T;
    fn into_node(self) -> Node {
        Node::Value(crate::Arg {
            ty: T::name(),
            value: Some(self.label().to_string()),
        })
    }
}

/// An enum value, bound by its label.
pub trait Label {
    fn label(&self) -> &'static str;
}
