//! Functions, aggregates and window functions.
//!
//! Arguments are checked against the parameter types of each function, with
//! implicit casts; results of strict functions are nullable when an argument
//! is.

use crate::Row;
use crate::ast::{self, Bound, Frame, Node, Window as WindowNode};
use crate::expr::{Base, Expr, Expression, Null, Operand, Operands, Order};
use crate::query::{FromItem, OrderBy};
use crate::schema::Alias;
use crate::types::*;
use std::marker::PhantomData;

/// A Rust value as an expression, bound as a parameter: `value(1) + 1`.
pub fn value<V: Operand>(value: V) -> Expr<V::Type> {
    Expr::new(value.into_node())
}

fn call<T>(name: &'static str, args: Vec<Node>) -> Expr<T> {
    Expr::new(Node::Call(name, args))
}

macro_rules! nullity {
    ($first:ty $(, $rest:ty)*) => { nullity!(@ <$first as PgType>::Null; $($rest),*) };
    (@ $acc:ty; ) => { $acc };
    (@ $acc:ty; $next:ty $(, $rest:ty)*) => {
        nullity!(@ <$acc as Nullity>::Or<<$next as PgType>::Null>; $($rest),*)
    };
}

/// Strict functions with fixed parameter types.
macro_rules! strict {
    ($($(#[$doc:meta])* $name:ident($($arg:ident: $ty:ty),*) -> $output:ty;)*) => {$(
        $(#[$doc])*
        #[allow(non_camel_case_types)]
        pub fn $name<$($arg: Operand),*>($($arg: $arg),*)
            -> Expr<<nullity!($($arg::Type),*) as Nullity>::Apply<$output>>
        where
            $(Base<$arg>: Coerce<$ty>,)*
        {
            call(stringify!($name), vec![$($arg.into_node()),*])
        }
    )*};
}

/// Strict functions whose result type depends on the argument type.
macro_rules! overloaded {
    ($($(#[$doc:meta])* $name:ident $trait:ident { $($input:ty => $output:ty,)* })*) => {$(
        #[doc = concat!("Argument types of `", stringify!($name), "`.")]
        pub trait $trait {
            type Output: PgType<Null = NotNull>;
        }
        $(
            impl $trait for $input {
                type Output = $output;
            }
        )*
        $(#[$doc])*
        pub fn $name<A: Operand>(arg: A) -> Expr<Like<<Base<A> as $trait>::Output, A::Type>>
        where
            Base<A>: $trait,
        {
            call(stringify!($name), vec![arg.into_node()])
        }
    )*};
}

strict! {
    /// `substr(text, start, count)`.
    substr(a: Text, b: Int4, c: Int4) -> Text;
    /// `power(x, y)`.
    power(a: Float8, b: Float8) -> Float8;
    /// `round(x, digits)`.
    round(a: Numeric, b: Int4) -> Numeric;
    /// `date_trunc(field, timestamp)`.
    date_trunc(a: Text, b: Timestamp) -> Timestamp;
    /// `to_char(timestamp, format)`.
    to_char(a: Timestamptz, b: Text) -> Text;
    /// `string_to_array(text, delimiter)`.
    string_to_array(a: Text, b: Text) -> Array<Text>;
    /// `jsonb_set(target, path, value)`.
    jsonb_set(a: Jsonb, b: Array<Text>, c: Jsonb) -> Jsonb;
    /// `jsonb_path_query_array(target, path)`.
    jsonb_path_query_array(a: Jsonb, b: Jsonpath) -> Jsonb;
    /// `jsonb_path_exists(target, path, vars)`.
    jsonb_path_exists(a: Jsonb, b: Jsonpath, c: Jsonb) -> Bool;
}

overloaded! {
    /// `length(text)`.
    length Length {
        Text => Int4,
        Bytea => Int4,
        Bit => Int4,
        Varbit => Int4,
        Tsvector => Int4,
    }
    /// `abs(x)`.
    abs Abs {
        Int2 => Int2,
        Int4 => Int4,
        Int8 => Int8,
        Numeric => Numeric,
        Float4 => Float4,
        Float8 => Float8,
    }
    /// `cardinality(array)`.
    cardinality Cardinality {
        Array<Int4> => Int4,
        Array<Text> => Int4,
    }
}

/// `lower(text)`, or the lower bound of a range or multirange.
pub fn lower<A: Operand>(arg: A) -> Expr<Like<<Base<A> as Bounded>::Output, A::Type>>
where
    Base<A>: Bounded,
{
    call("lower", vec![arg.into_node()])
}

/// `upper(text)`, or the upper bound of a range or multirange.
pub fn upper<A: Operand>(arg: A) -> Expr<Like<<Base<A> as Bounded>::Output, A::Type>>
where
    Base<A>: Bounded,
{
    call("upper", vec![arg.into_node()])
}

/// Text, whose case `lower` and `upper` change, and ranges and multiranges,
/// whose bounds they return, NULL when infinite.
pub trait Bounded {
    type Output: PgType;
}

impl Bounded for Text {
    type Output = Text;
}

impl<T: RangeSubtype> Bounded for Range<T> {
    type Output = Nullable<T>;
}

impl<T: RangeSubtype> Bounded for Multirange<T> {
    type Output = Nullable<T>;
}

/// Ranges and multiranges.
pub trait Ranged {
    type Subtype: RangeSubtype;
}

impl<T: RangeSubtype> Ranged for Range<T> {
    type Subtype = T;
}

impl<T: RangeSubtype> Ranged for Multirange<T> {
    type Subtype = T;
}

macro_rules! range_predicates {
    ($($(#[$doc:meta])* $name:ident)*) => {$(
        $(#[$doc])*
        pub fn $name<A: Operand>(range: A) -> Expr<Like<Bool, A::Type>>
        where
            Base<A>: Ranged,
        {
            call(stringify!($name), vec![range.into_node()])
        }
    )*};
}

range_predicates! {
    /// `isempty(range)`.
    isempty
    /// `lower_inc(range)`, whether the lower bound is inclusive.
    lower_inc
    /// `upper_inc(range)`, whether the upper bound is inclusive.
    upper_inc
}

/// `int4range(lower, upper)` and the other range constructors, by the type
/// of the bounds; a NULL bound is infinite.
pub fn range<L: Operand, U: Operand>(
    lower: L,
    upper: U,
) -> Expr<Strict<Range<Base<L>>, L::Type, U::Type>>
where
    Base<L>: RangeSubtype,
    Base<U>: Coerce<Base<L>>,
{
    call(Base::<L>::RANGE, vec![lower.into_node(), upper.into_node()])
}

/// `int4range(lower, upper, bounds)`, with bounds such as `"[]"` or `"(]"`.
pub fn range_bounds<L: Operand, U: Operand, B: Operand>(
    lower: L,
    upper: U,
    bounds: B,
) -> Expr<Strict<Range<Base<L>>, L::Type, U::Type>>
where
    Base<L>: RangeSubtype,
    Base<U>: Coerce<Base<L>>,
    Base<B>: Coerce<Text>,
{
    let args = vec![lower.into_node(), upper.into_node(), bounds.into_node()];
    call(Base::<L>::RANGE, args)
}

/// `int4multirange(range)` and the other multirange constructors.
pub fn multirange<R: Operand>(
    range: R,
) -> Expr<Like<Multirange<<Base<R> as Ranged>::Subtype>, R::Type>>
where
    Base<R>: Ranged,
{
    call(
        <<Base<R> as Ranged>::Subtype as RangeSubtype>::MULTIRANGE,
        vec![range.into_node()],
    )
}

/// `CURRENT_DATE`.
pub fn current_date() -> Expr<Date> {
    Expr::new(Node::Keyword("CURRENT_DATE"))
}

/// An aggregate call, which can be DISTINCT, ordered, filtered and used as a
/// window function with `over`.
pub struct Agg<T> {
    aggregate: ast::Aggregate,
    ty: PhantomData<fn() -> T>,
}

fn aggregate<T>(name: &'static str, args: Vec<Node>) -> Agg<T> {
    Agg {
        aggregate: ast::Aggregate {
            name,
            args,
            distinct: false,
            order: Vec::new(),
            within_group: Vec::new(),
            filter: None,
        },
        ty: PhantomData,
    }
}

impl<T: PgType> Agg<T> {
    /// `agg(DISTINCT ...)`.
    pub fn distinct(mut self) -> Self {
        self.aggregate.distinct = true;
        self
    }

    /// `agg(... ORDER BY ...)`.
    pub fn order_by<O: OrderBy>(mut self, order: O) -> Self {
        order.orders(&mut self.aggregate.order);
        self
    }

    /// `agg(...) FILTER (WHERE condition)`.
    pub fn filter<E: Operand>(mut self, condition: E) -> Self
    where
        Base<E>: Boolean,
    {
        self.aggregate.filter = Some(condition.into_node());
        self
    }

    /// `agg(...) OVER (window)`.
    pub fn over(self, window: Window) -> Expr<T> {
        Expr::new(Node::Over(Box::new(self.into_node()), Box::new(window.0)))
    }
}

impl<T: PgType> Operand for Agg<T> {
    type Type = T;
    fn into_node(self) -> Node {
        Node::Aggregate(Box::new(self.aggregate))
    }
}

impl<T: PgType> Expression for Agg<T> {}

/// `count(*)`.
pub fn count_star() -> Agg<Int8> {
    aggregate("count", Vec::new())
}

/// `count(x)`.
pub fn count<A: Operand>(arg: A) -> Agg<Int8> {
    aggregate("count", vec![arg.into_node()])
}

macro_rules! aggregates {
    ($($(#[$doc:meta])* $name:ident $trait:ident { $($input:ty => $output:ty,)* })*) => {$(
        #[doc = concat!("Argument types of `", stringify!($name), "`.")]
        pub trait $trait {
            type Output: PgType<Null = NotNull>;
        }
        $(
            impl $trait for $input {
                type Output = $output;
            }
        )*
        $(#[$doc])*
        pub fn $name<A: Operand>(arg: A) -> Agg<Nullable<<Base<A> as $trait>::Output>>
        where
            Base<A>: $trait,
        {
            aggregate(stringify!($name), vec![arg.into_node()])
        }
    )*};
}

aggregates! {
    /// `sum(x)`, NULL without rows.
    sum Sum {
        Int2 => Int8,
        Int4 => Int8,
        Int8 => Numeric,
        Numeric => Numeric,
        Float4 => Float4,
        Float8 => Float8,
        Money => Money,
        Interval => Interval,
    }
    /// `avg(x)`, NULL without rows.
    avg Avg {
        Int2 => Numeric,
        Int4 => Numeric,
        Int8 => Numeric,
        Numeric => Numeric,
        Float4 => Float8,
        Float8 => Float8,
        Interval => Interval,
    }
}

/// `min(x)`, NULL without rows.
pub fn min<A: Operand>(arg: A) -> Agg<Nullable<Base<A>>>
where
    Base<A>: crate::ops::Lt<Base<A>>,
{
    aggregate("min", vec![arg.into_node()])
}

/// `max(x)`, NULL without rows.
pub fn max<A: Operand>(arg: A) -> Agg<Nullable<Base<A>>>
where
    Base<A>: crate::ops::Lt<Base<A>>,
{
    aggregate("max", vec![arg.into_node()])
}

/// `bool_or(x)`.
pub fn bool_or<A: Operand>(arg: A) -> Agg<Nullable<Bool>>
where
    Base<A>: Boolean,
{
    aggregate("bool_or", vec![arg.into_node()])
}

/// `array_agg(x)`.
pub fn array_agg<A: Operand>(arg: A) -> Agg<Nullable<Array<Base<A>>>>
where
    Base<A>: Element,
{
    aggregate("array_agg", vec![arg.into_node()])
}

/// `string_agg(text, delimiter)`.
pub fn string_agg<A: Operand, D: Operand>(arg: A, delimiter: D) -> Agg<Nullable<Text>>
where
    Base<A>: Coerce<Text>,
    Base<D>: Coerce<Text>,
{
    aggregate("string_agg", vec![arg.into_node(), delimiter.into_node()])
}

/// A window definition, for `over`.
#[derive(Clone, Default)]
pub struct Window(WindowNode);

/// An empty window definition, `OVER ()`.
pub fn window() -> Window {
    Window::default()
}

impl Window {
    /// `PARTITION BY expressions`.
    pub fn partition_by<E: crate::query::Exprs>(mut self, expressions: E) -> Self {
        self.0.partition.extend(expressions.into_nodes());
        self
    }

    /// `ORDER BY ...`.
    pub fn order_by<O: OrderBy>(mut self, order: O) -> Self {
        order.orders(&mut self.0.order);
        self
    }

    fn frame(mut self, units: &'static str, start: FrameBound, end: FrameBound) -> Self {
        self.0.frame = Some(Frame {
            units,
            start: start.0,
            end: end.0,
            exclude: None,
        });
        self
    }

    /// `ROWS BETWEEN start AND end`.
    pub fn rows(self, start: FrameBound, end: FrameBound) -> Self {
        self.frame("ROWS", start, end)
    }

    /// `RANGE BETWEEN start AND end`.
    pub fn range(self, start: FrameBound, end: FrameBound) -> Self {
        self.frame("RANGE", start, end)
    }

    /// `GROUPS BETWEEN start AND end`.
    pub fn groups(self, start: FrameBound, end: FrameBound) -> Self {
        self.frame("GROUPS", start, end)
    }

    fn exclude(mut self, exclude: &'static str) -> Self {
        let frame = self.0.frame.as_mut().expect("a frame to exclude rows from");
        frame.exclude = Some(exclude);
        self
    }

    /// `EXCLUDE CURRENT ROW`.
    pub fn exclude_current_row(self) -> Self {
        self.exclude("CURRENT ROW")
    }

    /// `EXCLUDE GROUP`.
    pub fn exclude_group(self) -> Self {
        self.exclude("GROUP")
    }

    /// `EXCLUDE TIES`.
    pub fn exclude_ties(self) -> Self {
        self.exclude("TIES")
    }
}

/// A frame start or end.
pub struct FrameBound(Bound);

pub fn unbounded_preceding() -> FrameBound {
    FrameBound(Bound::UnboundedPreceding)
}

/// `offset PRECEDING`.
pub fn preceding<O: Operand>(offset: O) -> FrameBound {
    FrameBound(Bound::Preceding(offset.into_node()))
}

pub fn current_row() -> FrameBound {
    FrameBound(Bound::CurrentRow)
}

/// `offset FOLLOWING`.
pub fn following<O: Operand>(offset: O) -> FrameBound {
    FrameBound(Bound::Following(offset.into_node()))
}

pub fn unbounded_following() -> FrameBound {
    FrameBound(Bound::UnboundedFollowing)
}

/// A window function, which needs `over`.
pub struct WindowFn<T> {
    call: Node,
    ty: PhantomData<fn() -> T>,
}

impl<T> WindowFn<T> {
    /// `f(...) OVER (window)`.
    pub fn over(self, window: Window) -> Expr<T> {
        Expr::new(Node::Over(Box::new(self.call), Box::new(window.0)))
    }
}

fn window_fn<T>(name: &'static str, args: Vec<Node>) -> WindowFn<T> {
    WindowFn {
        call: Node::Call(name, args),
        ty: PhantomData,
    }
}

pub fn row_number() -> WindowFn<Int8> {
    window_fn("row_number", Vec::new())
}

pub fn rank() -> WindowFn<Int8> {
    window_fn("rank", Vec::new())
}

pub fn dense_rank() -> WindowFn<Int8> {
    window_fn("dense_rank", Vec::new())
}

pub fn percent_rank() -> WindowFn<Float8> {
    window_fn("percent_rank", Vec::new())
}

pub fn cume_dist() -> WindowFn<Float8> {
    window_fn("cume_dist", Vec::new())
}

/// `ntile(buckets)`.
pub fn ntile<N: Operand>(buckets: N) -> WindowFn<Int4>
where
    Base<N>: Coerce<Int4>,
{
    window_fn("ntile", vec![buckets.into_node()])
}

/// `lag(x)`, NULL on the first row.
pub fn lag<A: Operand>(arg: A) -> WindowFn<Nullable<Base<A>>> {
    window_fn("lag", vec![arg.into_node()])
}

/// `lead(x, offset, default)`.
pub fn lead<A: Operand, O: Operand, D: Operand>(
    arg: A,
    offset: O,
    default: D,
) -> WindowFn<Nullable<Base<A>>>
where
    Base<O>: Coerce<Int4>,
    Base<D>: Coerce<Base<A>>,
{
    window_fn(
        "lead",
        vec![arg.into_node(), offset.into_node(), default.into_node()],
    )
}

pub fn first_value<A: Operand>(arg: A) -> WindowFn<Nullable<Base<A>>> {
    window_fn("first_value", vec![arg.into_node()])
}

pub fn last_value<A: Operand>(arg: A) -> WindowFn<Nullable<Base<A>>> {
    window_fn("last_value", vec![arg.into_node()])
}

/// `nth_value(x, n)`.
pub fn nth_value<A: Operand, N: Operand>(arg: A, n: N) -> WindowFn<Nullable<Base<A>>>
where
    Base<N>: Coerce<Int4>,
{
    window_fn("nth_value", vec![arg.into_node(), n.into_node()])
}

/// An ordered-set or hypothetical-set aggregate waiting for `within_group`.
pub struct OrderedSet<F> {
    name: &'static str,
    args: Vec<Node>,
    result: PhantomData<F>,
}

/// The result type of an ordered-set aggregate over values of type `T`.
pub trait OrderedResult {
    type Output<T: PgType>: PgType;
}

/// Results of the type of the ordered values, as `percentile_disc`.
pub struct OfValues;

/// Results of a fixed type `T`, NULL without rows.
pub struct Fixed<T>(PhantomData<T>);

/// Results of a fixed type `T`, never NULL.
pub struct Always<T>(PhantomData<T>);

impl OrderedResult for OfValues {
    type Output<T: PgType> = Nullable<T::Base>;
}

impl<F: PgType<Null = NotNull>> OrderedResult for Fixed<F> {
    type Output<T: PgType> = Nullable<F>;
}

impl<F: PgType<Null = NotNull>> OrderedResult for Always<F> {
    type Output<T: PgType> = F;
}

impl<F: OrderedResult> OrderedSet<F> {
    /// `f(...) WITHIN GROUP (ORDER BY value)`.
    pub fn within_group<E: Expression>(self, value: E) -> Agg<F::Output<E::Type>> {
        self.within_group_by(value.asc_default())
    }

    fn within_group_by<T>(self, order: Order) -> Agg<T> {
        let mut agg = aggregate(self.name, self.args);
        agg.aggregate.within_group.push(order.0);
        agg
    }
}

trait AscDefault {
    fn asc_default(self) -> Order;
}

impl<E: Expression> AscDefault for E {
    fn asc_default(self) -> Order {
        Order(ast::Order {
            node: self.into_node(),
            direction: None,
            nulls: None,
        })
    }
}

fn ordered_set<F>(name: &'static str, args: Vec<Node>) -> OrderedSet<F> {
    OrderedSet {
        name,
        args,
        result: PhantomData,
    }
}

/// `percentile_cont(fraction) WITHIN GROUP (ORDER BY x)`.
pub fn percentile_cont<A: Operand>(fraction: A) -> OrderedSet<Fixed<Float8>>
where
    Base<A>: Coerce<Float8>,
{
    ordered_set("percentile_cont", vec![fraction.into_node()])
}

/// `percentile_disc(fraction) WITHIN GROUP (ORDER BY x)`.
pub fn percentile_disc<A: Operand>(fraction: A) -> OrderedSet<OfValues>
where
    Base<A>: Coerce<Float8>,
{
    ordered_set("percentile_disc", vec![fraction.into_node()])
}

/// `mode() WITHIN GROUP (ORDER BY x)`.
pub fn mode() -> OrderedSet<OfValues> {
    ordered_set("mode", Vec::new())
}

/// `rank(value) WITHIN GROUP (ORDER BY x)`, the rank a value would have.
pub fn hypothetical_rank<A: Operand>(value: A) -> OrderedSet<Always<Int8>> {
    ordered_set("rank", vec![value.into_node()])
}

/// `dense_rank(value) WITHIN GROUP (ORDER BY x)`.
pub fn hypothetical_dense_rank<A: Operand>(value: A) -> OrderedSet<Always<Int8>> {
    ordered_set("dense_rank", vec![value.into_node()])
}

/// `percent_rank(value) WITHIN GROUP (ORDER BY x)`.
pub fn hypothetical_percent_rank<A: Operand>(value: A) -> OrderedSet<Always<Float8>> {
    ordered_set("percent_rank", vec![value.into_node()])
}

/// `cume_dist(value) WITHIN GROUP (ORDER BY x)`.
pub fn hypothetical_cume_dist<A: Operand>(value: A) -> OrderedSet<Always<Float8>> {
    ordered_set("cume_dist", vec![value.into_node()])
}

/// `GROUPING(keys)`, of grouping keys.
pub fn grouping<K: Operands>(keys: K) -> Expr<Int4> {
    call("GROUPING", keys.into_nodes())
}

/// A set-returning function, usable in FROM, and in a SELECT list when it
/// returns a single column.
pub struct SetFn<R> {
    calls: Vec<Node>,
    rows_from: bool,
    ordinality: bool,
    row: PhantomData<fn() -> R>,
}

fn set_fn<R>(name: &'static str, args: Vec<Node>) -> SetFn<R> {
    SetFn {
        calls: vec![Node::Call(name, args)],
        rows_from: false,
        ordinality: false,
        row: PhantomData,
    }
}

impl<R: Row> SetFn<R> {
    /// `WITH ORDINALITY`, adding a row number column.
    pub fn with_ordinality(self) -> SetFn<<R as crate::Push<Int8>>::Out>
    where
        R: crate::Push<Int8>,
    {
        SetFn {
            calls: self.calls,
            rows_from: self.rows_from,
            ordinality: true,
            row: PhantomData,
        }
    }
}

impl<R: Row> FromItem for SetFn<R> {
    type Source = R::Columns;
    fn into_from(self) -> (ast::From, R::Columns) {
        let id = crate::query::next_id();
        let from = ast::From {
            source: ast::Source::Function {
                calls: self.calls,
                rows_from: self.rows_from,
                ordinality: self.ordinality,
            },
            rel: id,
            columns: R::LEN,
        };
        (from, R::columns(Alias(ast::Rel::Source(id))))
    }
}

impl<T: PgType> Operand for SetFn<(T,)> {
    type Type = T;
    fn into_node(self) -> Node {
        self.calls.into_iter().next().unwrap()
    }
}

/// `ROWS FROM (f, g, ...)`: the functions side by side, padded with NULLs.
pub fn rows_from<F: SetFns>(functions: F) -> SetFn<F::Row> {
    SetFn {
        calls: functions.calls(),
        rows_from: true,
        ordinality: false,
        row: PhantomData,
    }
}

/// Set-returning functions for `rows_from`.
pub trait SetFns {
    type Row: Row;
    #[doc(hidden)]
    fn calls(self) -> Vec<Node>;
}

impl<A: PgType, B: PgType> SetFns for (SetFn<(A,)>, SetFn<(B,)>) {
    type Row = (Nullable<A::Base>, Nullable<B::Base>);
    fn calls(self) -> Vec<Node> {
        vec![self.0.into_node(), self.1.into_node()]
    }
}

/// `generate_series(start, stop)`.
pub fn generate_series<A: Operand, B: Operand>(
    start: A,
    stop: B,
) -> SetFn<(Strict<Base<A>, A::Type, B::Type>,)>
where
    Base<A>: Series,
    Base<B>: Coerce<Base<A>>,
{
    set_fn("generate_series", vec![start.into_node(), stop.into_node()])
}

/// Types with `generate_series(start, stop)`.
pub trait Series: PgType<Null = NotNull> {}
impl Series for Int4 {}
impl Series for Int8 {}
impl Series for Numeric {}

/// `unnest(array)`, or the ranges of a multirange.
pub fn unnest<A: Operand>(arg: A) -> SetFn<(Like<<Base<A> as Unnest>::Output, A::Type>,)>
where
    Base<A>: Unnest,
{
    set_fn("unnest", vec![arg.into_node()])
}

/// Arrays and multiranges, the arguments of `unnest`.
pub trait Unnest {
    type Output: PgType<Null = NotNull>;
}

impl<T: Element> Unnest for Array<T> {
    type Output = T;
}

impl<T: RangeSubtype> Unnest for Multirange<T> {
    type Output = Range<T>;
}

/// `jsonb_path_query(target, path)`.
pub fn jsonb_path_query<A: Operand, P: Operand>(
    target: A,
    path: P,
) -> SetFn<(Like<Jsonb, A::Type>,)>
where
    Base<A>: Coerce<Jsonb>,
    Base<P>: Coerce<Jsonpath>,
{
    set_fn(
        "jsonb_path_query",
        vec![target.into_node(), path.into_node()],
    )
}

/// `coalesce(a, b)`, NULL only when both are.
pub fn coalesce<A: Operand, B: Operand>(a: A, b: B) -> Expr<Both<Null<A>, Null<B>, Base<A>>>
where
    Base<B>: Coerce<Base<A>>,
{
    call("coalesce", vec![a.into_node(), b.into_node()])
}

/// `range_agg(range)`, the union of ranges or multiranges.
pub fn range_agg<A: Operand>(arg: A) -> Agg<Nullable<Multirange<<Base<A> as Ranged>::Subtype>>>
where
    Base<A>: Ranged,
{
    aggregate("range_agg", vec![arg.into_node()])
}

/// `range_intersect_agg(range)`.
pub fn range_intersect_agg<A: Operand>(arg: A) -> Agg<Nullable<Base<A>>>
where
    Base<A>: Ranged,
{
    aggregate("range_intersect_agg", vec![arg.into_node()])
}

/// `nullif(a, b)`, NULL when `a = b`.
pub fn nullif<A: Operand, B: Operand>(a: A, b: B) -> Expr<Nullable<Base<A>>>
where
    Base<A>: crate::ops::Eq<Base<B>>,
{
    call("nullif", vec![a.into_node(), b.into_node()])
}

/// `greatest(a, b)`, ignoring NULLs.
pub fn greatest<A: Operand, B: Operand>(a: A, b: B) -> Expr<Both<Null<A>, Null<B>, Base<A>>>
where
    Base<B>: Coerce<Base<A>>,
{
    call("greatest", vec![a.into_node(), b.into_node()])
}

/// `least(a, b)`, ignoring NULLs.
pub fn least<A: Operand, B: Operand>(a: A, b: B) -> Expr<Both<Null<A>, Null<B>, Base<A>>>
where
    Base<B>: Coerce<Base<A>>,
{
    call("least", vec![a.into_node(), b.into_node()])
}

/// `concat_ws(separator, values...)`, skipping NULL values.
pub fn concat_ws<S: Operand, V: Operands>(separator: S, values: V) -> Expr<Like<Text, S::Type>>
where
    Base<S>: Coerce<Text>,
{
    let mut args = vec![separator.into_node()];
    args.extend(values.into_nodes());
    call("concat_ws", args)
}

/// `format(format, values...)`, with NULL values as empty strings.
pub fn format<F: Operand, V: Operands>(format: F, values: V) -> Expr<Like<Text, F::Type>>
where
    Base<F>: Coerce<Text>,
{
    let mut args = vec![format.into_node()];
    args.extend(values.into_nodes());
    call("format", args)
}

/// `pg_typeof(value)`.
pub fn pg_typeof<A: Operand>(value: A) -> Expr<Regtype> {
    call("pg_typeof", vec![value.into_node()])
}

/// A NULL of type `T`, `CAST(NULL AS T)`.
pub fn typed_null<T: PgType<Null = NotNull>>() -> Expr<Nullable<T>> {
    Expr::new(Node::Cast(Box::new(Node::Null), T::name()))
}

/// `enum_first(value)`, the first value of the enum type.
pub fn enum_first<E: Operand>(value: E) -> Expr<Nullable<Base<E>>>
where
    Base<E>: Enum,
{
    call("enum_first", vec![value.into_node()])
}

/// `enum_last(value)`, the last value of the enum type.
pub fn enum_last<E: Operand>(value: E) -> Expr<Nullable<Base<E>>>
where
    Base<E>: Enum,
{
    call("enum_last", vec![value.into_node()])
}

/// `enum_range(value)`, all values of the enum type in order.
pub fn enum_range<E: Operand>(value: E) -> Expr<Nullable<Array<Base<E>>>>
where
    Base<E>: Enum,
{
    call("enum_range", vec![value.into_node()])
}

/// `enum_range(low, high)`, the values between two, a NULL bound being open.
pub fn enum_range_between<E: Operand, H: Operand>(low: E, high: H) -> Expr<Nullable<Array<Base<E>>>>
where
    Base<E>: Enum,
    Base<H>: Coerce<Base<E>>,
{
    call("enum_range", vec![low.into_node(), high.into_node()])
}

/// A field of a date or time for `extract`.
#[derive(Clone, Copy, Debug)]
pub enum DatePart {
    Century,
    Day,
    Decade,
    Dow,
    Doy,
    Epoch,
    Hour,
    Isodow,
    Isoyear,
    Julian,
    Microseconds,
    Millennium,
    Milliseconds,
    Minute,
    Month,
    Quarter,
    Second,
    Timezone,
    TimezoneHour,
    TimezoneMinute,
    Week,
    Year,
}

impl DatePart {
    fn keyword(self) -> &'static str {
        match self {
            DatePart::Century => "century",
            DatePart::Day => "day",
            DatePart::Decade => "decade",
            DatePart::Dow => "dow",
            DatePart::Doy => "doy",
            DatePart::Epoch => "epoch",
            DatePart::Hour => "hour",
            DatePart::Isodow => "isodow",
            DatePart::Isoyear => "isoyear",
            DatePart::Julian => "julian",
            DatePart::Microseconds => "microseconds",
            DatePart::Millennium => "millennium",
            DatePart::Milliseconds => "milliseconds",
            DatePart::Minute => "minute",
            DatePart::Month => "month",
            DatePart::Quarter => "quarter",
            DatePart::Second => "second",
            DatePart::Timezone => "timezone",
            DatePart::TimezoneHour => "timezone_hour",
            DatePart::TimezoneMinute => "timezone_minute",
            DatePart::Week => "week",
            DatePart::Year => "year",
        }
    }
}

/// Types `extract` reads fields from.
pub trait Datelike {}
impl Datelike for Date {}
impl Datelike for Time {}
impl Datelike for Timetz {}
impl Datelike for Timestamp {}
impl Datelike for Timestamptz {}
impl Datelike for Interval {}

/// `extract(part FROM value)`.
pub fn extract<A: Operand>(part: DatePart, value: A) -> Expr<Like<Numeric, A::Type>>
where
    Base<A>: Datelike,
{
    Expr::new(Node::Syntax(
        "extract",
        vec![
            Node::Keyword(part.keyword()),
            Node::Keyword("FROM"),
            value.into_node(),
        ],
    ))
}

/// `make_date(year, month, day)`, in named notation.
pub fn make_date<Y: Operand, M: Operand, D: Operand>(
    year: Y,
    month: M,
    day: D,
) -> Expr<Strict<Like<Date, Y::Type>, M::Type, D::Type>>
where
    Base<Y>: Coerce<Int4>,
    Base<M>: Coerce<Int4>,
    Base<D>: Coerce<Int4>,
{
    call(
        "make_date",
        vec![
            Node::Named("year", Box::new(year.into_node())),
            Node::Named("month", Box::new(month.into_node())),
            Node::Named("day", Box::new(day.into_node())),
        ],
    )
}

/// `make_interval(...)`, its fields given by name; the others are zero.
pub fn make_interval() -> MakeInterval<NotNull> {
    MakeInterval(Vec::new(), PhantomData)
}

/// A `make_interval` call, nullable when a field is.
pub struct MakeInterval<N>(Vec<Node>, PhantomData<N>);

macro_rules! interval_fields {
    ($($field:ident: $ty:ty),*) => {$(
        #[doc = concat!("`", stringify!($field), " => value`.")]
        pub fn $field<A: Operand>(mut self, value: A) -> MakeInterval<N::Or<Null<A>>>
        where
            Base<A>: Coerce<$ty>,
        {
            self.0.push(Node::Named(stringify!($field), Box::new(value.into_node())));
            MakeInterval(self.0, PhantomData)
        }
    )*};
}

impl<N: Nullity> MakeInterval<N> {
    interval_fields!(years: Int4, months: Int4, weeks: Int4, days: Int4, hours: Int4, mins: Int4, secs: Float8);
}

impl<N: Nullity> Operand for MakeInterval<N> {
    type Type = N::Apply<Interval>;
    fn into_node(self) -> Node {
        Node::Call("make_interval", self.0)
    }
}

impl<N: Nullity> Expression for MakeInterval<N> {}

/// `ARRAY[a, b, ...]`, of elements of the same type.
pub fn array<E: Elements>(elements: E) -> Expr<Array<E::Element>> {
    Expr::new(Node::Array(elements.into_nodes()))
}

/// The elements of an array constructor: a tuple of operands of one type.
pub trait Elements: Operands {
    type Element: Element;
}

/// `CASE WHEN condition THEN value ...`; add branches with `when`, finish
/// with `otherwise` or use as is, NULL when no branch matches.
pub fn when<C: Operand, V: Operand>(condition: C, then: V) -> Case<Searched, Base<V>, Null<V>>
where
    Base<C>: Boolean,
{
    Case {
        operand: None,
        whens: vec![(condition.into_node(), then.into_node())],
        ty: PhantomData,
    }
}

/// `CASE operand WHEN value THEN result ...`.
pub fn case<O: Operand>(operand: O) -> CaseOf<Base<O>> {
    CaseOf(operand.into_node(), PhantomData)
}

/// A simple CASE waiting for its first branch.
pub struct CaseOf<O>(Node, PhantomData<O>);

impl<O> CaseOf<O> {
    pub fn when<V: Operand, R: Operand>(self, value: V, then: R) -> Case<Of<O>, Base<R>, Null<R>>
    where
        O: crate::ops::Eq<Base<V>>,
    {
        Case {
            operand: Some(self.0),
            whens: vec![(value.into_node(), then.into_node())],
            ty: PhantomData,
        }
    }
}

/// A searched CASE.
pub struct Searched;

/// A simple CASE on an operand of type `O`.
pub struct Of<O>(PhantomData<O>);

/// A CASE expression with results of type `T` and nullability `N` so far.
pub struct Case<K, T, N> {
    operand: Option<Node>,
    whens: Vec<(Node, Node)>,
    ty: PhantomData<fn() -> (K, T, N)>,
}

impl<K, T, N: Nullity> Case<K, T, N> {
    fn branch<C: Operand, V: Operand>(mut self, when: C, then: V) -> Case<K, T, N::Or<Null<V>>> {
        self.whens.push((when.into_node(), then.into_node()));
        Case {
            operand: self.operand,
            whens: self.whens,
            ty: PhantomData,
        }
    }

    /// `ELSE value END`.
    pub fn otherwise<V: Operand>(self, value: V) -> Expr<<N::Or<Null<V>> as Nullity>::Apply<T>>
    where
        T: PgType,
        Base<V>: Coerce<T>,
    {
        Expr::new(Node::Case(
            self.operand.map(Box::new),
            self.whens,
            Some(Box::new(value.into_node())),
        ))
    }
}

impl<T, N: Nullity> Case<Searched, T, N> {
    pub fn when<C: Operand, V: Operand>(
        self,
        condition: C,
        then: V,
    ) -> Case<Searched, T, N::Or<Null<V>>>
    where
        Base<C>: Boolean,
        Base<V>: Coerce<T>,
    {
        self.branch(condition, then)
    }
}

impl<O, T, N: Nullity> Case<Of<O>, T, N> {
    pub fn when<V: Operand, R: Operand>(self, value: V, then: R) -> Case<Of<O>, T, N::Or<Null<R>>>
    where
        O: crate::ops::Eq<Base<V>>,
        Base<R>: Coerce<T>,
    {
        self.branch(value, then)
    }
}

impl<K, T: PgType<Null = NotNull>, N: Nullity> Operand for Case<K, T, N> {
    type Type = Nullable<T>;
    fn into_node(self) -> Node {
        Node::Case(self.operand.map(Box::new), self.whens, None)
    }
}

impl<K, T: PgType<Null = NotNull>, N: Nullity> Expression for Case<K, T, N> {}

/// `jsonb_build_object(key, value, ...)` from `(key, value)` pairs.
pub fn jsonb_build_object<P: Pairs>(pairs: P) -> Expr<Jsonb> {
    call("jsonb_build_object", pairs.into_nodes(None))
}

/// `JSON_OBJECT(key : value, ...)` from `(key, value)` pairs.
pub fn json_object<P: Pairs>(pairs: P) -> Expr<Json> {
    Expr::new(Node::Syntax("JSON_OBJECT", pairs.into_nodes(Some(":"))))
}

/// `JSON_ARRAY(values...)`.
pub fn json_array<V: Operands>(values: V) -> Expr<Json> {
    let mut parts = Vec::new();
    for (i, value) in values.into_nodes().into_iter().enumerate() {
        if i > 0 {
            parts.push(Node::Keyword(","));
        }
        parts.push(value);
    }
    Expr::new(Node::Syntax("JSON_ARRAY", parts))
}

/// Key-value pairs: a tuple of `(key, value)` tuples with text keys.
pub trait Pairs {
    #[doc(hidden)]
    fn into_nodes(self, separator: Option<&'static str>) -> Vec<Node>;
}

macro_rules! pairs {
    ($(($($k:ident $v:ident $i:tt),*))*) => {$(
        impl<$($k: Operand, $v: Operand),*> Pairs for ($(($k, $v),)*)
        where
            $(Base<$k>: Coerce<Text>,)*
        {
            fn into_nodes(self, separator: Option<&'static str>) -> Vec<Node> {
                let pairs = vec![$((self.$i.0.into_node(), self.$i.1.into_node())),*];
                let mut nodes = Vec::new();
                for (i, (key, value)) in pairs.into_iter().enumerate() {
                    if let Some(separator) = separator {
                        if i > 0 {
                            nodes.push(Node::Keyword(","));
                        }
                        nodes.extend([key, Node::Keyword(separator), value]);
                    } else {
                        nodes.extend([key, value]);
                    }
                }
                nodes
            }
        }
    )*};
}

pairs! {
    (K0 V0 0)
    (K0 V0 0, K1 V1 1)
    (K0 V0 0, K1 V1 1, K2 V2 2)
    (K0 V0 0, K1 V1 1, K2 V2 2, K3 V3 3)
    (K0 V0 0, K1 V1 1, K2 V2 2, K3 V3 3, K4 V4 4)
    (K0 V0 0, K1 V1 1, K2 V2 2, K3 V3 3, K4 V4 4, K5 V5 5)
}

/// A JSON path, bound as a `jsonpath` parameter.
pub fn jsonpath(path: &str) -> Expr<Jsonpath> {
    crate::literal(path)
}

/// Types the SQL/JSON functions and predicates accept as JSON input.
pub trait JsonInput {}
impl JsonInput for Json {}
impl JsonInput for Jsonb {}
impl JsonInput for Text {}
impl JsonInput for Varchar {}
impl JsonInput for Bytea {}

/// `x IS JSON` and its variants.
#[allow(clippy::wrong_self_convention)]
pub trait JsonPredicates: Operand + Sized
where
    Base<Self>: JsonInput,
{
    fn is_json(self) -> Expr<Like<Bool, Self::Type>> {
        json_test(self, "IS JSON")
    }

    fn is_not_json(self) -> Expr<Like<Bool, Self::Type>> {
        json_test(self, "IS NOT JSON")
    }

    fn is_json_object(self) -> Expr<Like<Bool, Self::Type>> {
        json_test(self, "IS JSON OBJECT")
    }

    fn is_json_array(self) -> Expr<Like<Bool, Self::Type>> {
        json_test(self, "IS JSON ARRAY")
    }

    fn is_json_scalar(self) -> Expr<Like<Bool, Self::Type>> {
        json_test(self, "IS JSON SCALAR")
    }

    fn is_json_with_unique_keys(self) -> Expr<Like<Bool, Self::Type>> {
        json_test(self, "IS JSON WITH UNIQUE KEYS")
    }
}

impl<E: Expression> JsonPredicates for E where Base<E>: JsonInput {}

fn json_test<E: Operand>(operand: E, test: &'static str) -> Expr<Like<Bool, E::Type>> {
    Expr::new(Node::Seq(vec![operand.into_node(), Node::Keyword(test)]))
}

/// A SQL/JSON query function, `JSON_EXISTS`, `JSON_QUERY` or `JSON_VALUE`,
/// with its clauses; results are NULL when the path finds nothing.
pub struct JsonQuery<T> {
    name: &'static str,
    parts: Vec<Node>,
    ty: PhantomData<fn() -> T>,
}

fn json_query_fn<T, A: Operand, P: Operand>(name: &'static str, input: A, path: P) -> JsonQuery<T> {
    JsonQuery {
        name,
        parts: vec![input.into_node(), Node::Keyword(","), path.into_node()],
        ty: PhantomData,
    }
}

/// `JSON_EXISTS(input, path)`.
pub fn json_exists<A: Operand, P: Operand>(input: A, path: P) -> JsonQuery<Bool>
where
    Base<A>: JsonInput,
    Base<P>: Coerce<Jsonpath>,
{
    json_query_fn("JSON_EXISTS", input, path)
}

/// `JSON_QUERY(input, path)`, a JSON fragment.
pub fn json_query<A: Operand, P: Operand>(input: A, path: P) -> JsonQuery<Jsonb>
where
    Base<A>: JsonInput,
    Base<P>: Coerce<Jsonpath>,
{
    json_query_fn("JSON_QUERY", input, path)
}

/// `JSON_VALUE(input, path)`, a scalar as text.
pub fn json_value<A: Operand, P: Operand>(input: A, path: P) -> JsonQuery<Text>
where
    Base<A>: JsonInput,
    Base<P>: Coerce<Jsonpath>,
{
    json_query_fn("JSON_VALUE", input, path)
}

impl<T: PgType<Null = NotNull>> JsonQuery<T> {
    fn clause(mut self, keywords: &'static str) -> Self {
        self.parts.push(Node::Keyword(keywords));
        self
    }

    /// `RETURNING type`.
    pub fn returning<R: PgType<Null = NotNull>>(mut self) -> JsonQuery<R> {
        self.parts.push(Node::Keyword("RETURNING"));
        self.parts.push(Node::Type(R::name()));
        JsonQuery {
            name: self.name,
            parts: self.parts,
            ty: PhantomData,
        }
    }

    /// `WITH WRAPPER`, wrapping the results in an array.
    pub fn with_wrapper(self) -> Self {
        self.clause("WITH WRAPPER")
    }

    /// `DEFAULT 'value' ON EMPTY`, a constant in the input form of the
    /// result type, as the grammar requires.
    pub fn default_on_empty(mut self, value: &'static str) -> Self {
        self.parts.push(Node::Keyword("DEFAULT"));
        self.parts.push(Node::Literal(value));
        self.clause("ON EMPTY")
    }

    /// `DEFAULT 'value' ON ERROR`, a constant like `default_on_empty`.
    pub fn default_on_error(mut self, value: &'static str) -> Self {
        self.parts.push(Node::Keyword("DEFAULT"));
        self.parts.push(Node::Literal(value));
        self.clause("ON ERROR")
    }

    /// `EMPTY ARRAY ON EMPTY`.
    pub fn empty_array_on_empty(self) -> Self {
        self.clause("EMPTY ARRAY ON EMPTY")
    }
}

impl<T: PgType<Null = NotNull>> Operand for JsonQuery<T> {
    type Type = Nullable<T>;
    fn into_node(self) -> Node {
        Node::Syntax(self.name, self.parts)
    }
}

impl<T: PgType<Null = NotNull>> Expression for JsonQuery<T> {}

/// `JSON_SERIALIZE(value)`, as text.
pub fn json_serialize<A: Operand>(value: A) -> Expr<Like<Text, A::Type>>
where
    Base<A>: JsonInput,
{
    Expr::new(Node::Syntax("JSON_SERIALIZE", vec![value.into_node()]))
}

/// `JSON_SERIALIZE(value RETURNING type)`.
pub fn json_serialize_as<T, A: Operand>(value: A) -> Expr<Like<T, A::Type>>
where
    T: PgType<Null = NotNull> + JsonInput,
    Base<A>: JsonInput,
{
    Expr::new(Node::Syntax(
        "JSON_SERIALIZE",
        vec![
            value.into_node(),
            Node::Keyword("RETURNING"),
            Node::Type(T::name()),
        ],
    ))
}

/// `JSON_SCALAR(value)`.
pub fn json_scalar<A: Operand>(value: A) -> Expr<Like<Json, A::Type>> {
    Expr::new(Node::Syntax("JSON_SCALAR", vec![value.into_node()]))
}

/// `JSON(text)`, parsing JSON text.
pub fn json<A: Operand>(text: A) -> Expr<Like<Json, A::Type>>
where
    Base<A>: JsonInput,
{
    Expr::new(Node::Syntax("JSON", vec![text.into_node()]))
}

/// `jsonb_agg(x)`.
pub fn jsonb_agg<A: Operand>(value: A) -> Agg<Nullable<Jsonb>> {
    aggregate("jsonb_agg", vec![value.into_node()])
}

/// `json_agg(x)`.
pub fn json_agg<A: Operand>(value: A) -> Agg<Nullable<Json>> {
    aggregate("json_agg", vec![value.into_node()])
}

/// `jsonb_object_agg(key, value)`.
pub fn jsonb_object_agg<K: Operand, V: Operand>(key: K, value: V) -> Agg<Nullable<Jsonb>>
where
    Base<K>: Coerce<Text>,
{
    aggregate("jsonb_object_agg", vec![key.into_node(), value.into_node()])
}

/// `JSON_TABLE(input, path COLUMNS (...))`, a FROM item with a column per
/// path; paths are constants, as the grammar requires.
pub fn json_table<A: Operand, C: JsonColumns>(
    input: A,
    path: &'static str,
    columns: C,
) -> SetFn<C::Row>
where
    Base<A>: JsonInput,
{
    let mut parts = vec![
        input.into_node(),
        Node::Keyword(","),
        Node::Literal(path),
        Node::Keyword("COLUMNS ("),
    ];
    parts.extend(columns.into_nodes());
    parts.push(Node::Keyword(")"));
    SetFn {
        calls: vec![Node::Syntax("JSON_TABLE", parts)],
        rows_from: false,
        ordinality: false,
        row: PhantomData,
    }
}

/// A `JSON_TABLE` column of type `T` at a constant path.
pub struct JsonColumn<T> {
    path: &'static str,
    ty: PhantomData<T>,
}

/// `name type PATH 'path'`, a JSON_TABLE column.
pub fn json_column<T: PgType<Null = NotNull>>(path: &'static str) -> JsonColumn<T> {
    JsonColumn {
        path,
        ty: PhantomData,
    }
}

/// The columns of a JSON_TABLE.
pub trait JsonColumns {
    type Row: Row;
    #[doc(hidden)]
    fn into_nodes(self) -> Vec<Node>;
}

macro_rules! json_columns {
    ($(($($t:ident $i:tt),*))*) => {$(
        impl<$($t: PgType<Null = NotNull>),*> JsonColumns for ($(JsonColumn<$t>,)*) {
            type Row = ($(Nullable<$t>,)*);
            fn into_nodes(self) -> Vec<Node> {
                let columns = [$((self.$i.path, $t::name())),*];
                let mut nodes = Vec::new();
                for (i, (path, ty)) in columns.into_iter().enumerate() {
                    if i > 0 {
                        nodes.push(Node::Keyword(","));
                    }
                    nodes.push(Node::Keyword(COLUMN_NAMES[i]));
                    nodes.push(Node::Type(ty));
                    nodes.push(Node::Keyword("PATH"));
                    nodes.push(Node::Literal(path));
                }
                nodes
            }
        }
    )*};
}

const COLUMN_NAMES: [&str; 8] = ["c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8"];

json_columns! {
    (T0 0)
    (T0 0, T1 1)
    (T0 0, T1 1, T2 2)
    (T0 0, T1 1, T2 2, T3 3)
    (T0 0, T1 1, T2 2, T3 3, T4 4)
    (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5)
    (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6)
    (T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7)
}
