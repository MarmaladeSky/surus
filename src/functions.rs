//! PostgreSQL functions, aggregates, window functions and special
//! expression syntax (`ROW`, `ARRAY`, SQL/JSON).

use crate::expr::*;
use crate::query::*;
use crate::types::*;
use std::marker::PhantomData;

fn call<T, N, S>(name: &'static str, args: Vec<Node>) -> Expr<T, N, S> {
    Expr::new(Node::call(name, args))
}

/// Nullability of a strict function of two arguments.
type Or<A, B> = <A as Nullability>::Or<B>;

// ---------------------------------------------------------------------------
// Aggregates

/// An aggregate call; add `DISTINCT`, `ORDER BY`, `FILTER` or `OVER`.
pub struct Agg<T, N, S> {
    call: Call,
    _type: PhantomData<fn() -> (T, N, S)>,
}

fn aggregate<T, N, S>(name: &'static str, args: Vec<Node>) -> Agg<T, N, S> {
    Agg {
        call: Call {
            name,
            args,
            ..Call::default()
        },
        _type: PhantomData,
    }
}

impl<T, N, S> Agg<T, N, S> {
    fn retype<S2>(self) -> Agg<T, N, S2> {
        Agg {
            call: self.call,
            _type: PhantomData,
        }
    }

    /// `agg(DISTINCT ...)`
    pub fn distinct(mut self) -> Self {
        self.call.distinct = true;
        self
    }

    /// `agg(... ORDER BY keys)`
    pub fn order_by<O: OrderBy>(mut self, keys: O) -> Agg<T, N, (S, O::Scope)> {
        keys.push_keys(&mut self.call.order);
        self.retype()
    }

    /// `agg(...) FILTER (WHERE condition)`
    pub fn filter<P: Predicate>(mut self, condition: P) -> Agg<T, N, (S, P::Scope)> {
        self.call.filter = Some(condition.into_node());
        self.retype()
    }

    /// `agg(...) OVER window`
    pub fn over<W: IntoWindow>(mut self, window: W) -> Expr<T, N, (S, W::Scope)> {
        self.call.over = Some(window.into_window());
        Expr::new(Node::Call(Box::new(self.call)))
    }
}

impl<T: SqlType, N: Nullability, S> Expression for Agg<T, N, S> {
    type Sql = T;
    type Null = N;
    type Scope = S;
    fn into_node(self) -> Node {
        Node::Call(Box::new(self.call))
    }
}

rust_operators!(Agg<T, N, S>);

/// `count(*)`
pub fn count_star() -> Agg<Int8, NotNull, ()> {
    aggregate("count", vec![Node::Keyword("*")])
}

/// `count(value)`
pub fn count<E: Expression>(value: E) -> Agg<Int8, NotNull, E::Scope> {
    aggregate("count", vec![value.into_node()])
}

/// Result types of `sum` and `avg`.
pub trait Summable {
    type Sum: SqlType;
    type Avg: SqlType;
}

macro_rules! summable {
    ($($t:ty => $sum:ty, $avg:ty);* $(;)?) => {$(
        impl Summable for $t { type Sum = $sum; type Avg = $avg; }
    )*};
}

summable! {
    Int2 => Int8, Numeric; Int4 => Int8, Numeric; Int8 => Numeric, Numeric;
    Numeric => Numeric, Numeric; Float4 => Float4, Float8; Float8 => Float8, Float8;
    Interval => Interval, Interval; Money => Money, Money;
}

/// `sum(value)`
pub fn sum<E: Expression>(value: E) -> Agg<<E::Sql as Summable>::Sum, Nullable, E::Scope>
where
    E::Sql: Summable,
{
    aggregate("sum", vec![value.into_node()])
}

/// `avg(value)`
pub fn avg<E: Expression>(value: E) -> Agg<<E::Sql as Summable>::Avg, Nullable, E::Scope>
where
    E::Sql: Summable,
{
    aggregate("avg", vec![value.into_node()])
}

/// `min(value)`
pub fn min<E: Expression>(value: E) -> Agg<E::Sql, Nullable, E::Scope>
where
    E::Sql: Operator<crate::op::Lt, E::Sql>,
{
    aggregate("min", vec![value.into_node()])
}

/// `max(value)`
pub fn max<E: Expression>(value: E) -> Agg<E::Sql, Nullable, E::Scope>
where
    E::Sql: Operator<crate::op::Lt, E::Sql>,
{
    aggregate("max", vec![value.into_node()])
}

/// `bool_and(value)`
pub fn bool_and<E: Predicate>(value: E) -> Agg<Bool, Nullable, E::Scope> {
    aggregate("bool_and", vec![value.into_node()])
}

/// `bool_or(value)`
pub fn bool_or<E: Predicate>(value: E) -> Agg<Bool, Nullable, E::Scope> {
    aggregate("bool_or", vec![value.into_node()])
}

/// `array_agg(value)`
pub fn array_agg<E: Expression>(value: E) -> Agg<Array<E::Sql>, Nullable, E::Scope> {
    aggregate("array_agg", vec![value.into_node()])
}

/// `string_agg(value, separator)`
pub fn string_agg<E: Arg<Text>, D: Arg<Text>>(
    value: E,
    separator: D,
) -> Agg<Text, Nullable, (E::Scope, D::Scope)> {
    aggregate("string_agg", vec![value.into_arg(), separator.into_arg()])
}

/// `json_agg(value)`
pub fn json_agg<E: Expression>(value: E) -> Agg<Json, Nullable, E::Scope> {
    aggregate("json_agg", vec![value.into_node()])
}

/// `jsonb_agg(value)`
pub fn jsonb_agg<E: Expression>(value: E) -> Agg<Jsonb, Nullable, E::Scope> {
    aggregate("jsonb_agg", vec![value.into_node()])
}

/// `jsonb_object_agg(key, value)`
pub fn jsonb_object_agg<K: Arg<Text>, V: Expression>(
    key: K,
    value: V,
) -> Agg<Jsonb, Nullable, (K::Scope, V::Scope)> {
    aggregate("jsonb_object_agg", vec![key.into_arg(), value.into_node()])
}

/// Range and multirange types aggregated by `range_agg`.
pub trait RangeLike {
    type Element: RangeElement;
}
impl<T: RangeElement> RangeLike for Range<T> {
    type Element = T;
}
impl<T: RangeElement> RangeLike for Multirange<T> {
    type Element = T;
}

/// `range_agg(range)`
pub fn range_agg<E: Expression>(
    value: E,
) -> Agg<Multirange<<E::Sql as RangeLike>::Element>, Nullable, E::Scope>
where
    E::Sql: RangeLike,
{
    aggregate("range_agg", vec![value.into_node()])
}

/// `range_intersect_agg(range)`
pub fn range_intersect_agg<E: Expression>(value: E) -> Agg<E::Sql, Nullable, E::Scope>
where
    E::Sql: RangeLike,
{
    aggregate("range_intersect_agg", vec![value.into_node()])
}

/// `GROUPING(keys)`
pub fn grouping<E: Exprs>(keys: E) -> Expr<Int4, NotNull, E::Scope> {
    call("GROUPING", keys.into_nodes())
}

/// An ordered-set or hypothetical-set aggregate awaiting `WITHIN GROUP`.
pub struct WithinGroup<K, A, S> {
    call: Call,
    _type: PhantomData<fn() -> (K, A, S)>,
}

fn within_group<K, A, S>(name: &'static str, args: Vec<Node>) -> WithinGroup<K, A, S> {
    WithinGroup {
        call: Call {
            name,
            args,
            ..Call::default()
        },
        _type: PhantomData,
    }
}

impl<K, A, S> WithinGroup<K, A, S> {
    fn order<T, N, E: Expression>(mut self, key: E) -> Agg<T, N, (S, E::Scope)> {
        self.call.within_group = vec![key.into_node()];
        Agg {
            call: self.call,
            _type: PhantomData,
        }
    }
}

/// Kinds of ordered-set aggregates.
pub struct Continuous;
pub struct Discrete;
pub struct Hypothetical<T>(PhantomData<T>);

impl<S> WithinGroup<Continuous, (), S> {
    /// `WITHIN GROUP (ORDER BY key)`
    pub fn within_group<E: Arg<Float8> + Expression>(
        self,
        key: E,
    ) -> Agg<Float8, Nullable, (S, <E as Expression>::Scope)> {
        self.order(key)
    }
}

impl<S> WithinGroup<Discrete, (), S> {
    /// `WITHIN GROUP (ORDER BY key)`
    pub fn within_group<E: Expression>(self, key: E) -> Agg<E::Sql, Nullable, (S, E::Scope)> {
        self.order(key)
    }
}

impl<T: SqlType, A, S> WithinGroup<Hypothetical<T>, A, S> {
    /// `WITHIN GROUP (ORDER BY key)`; the hypothetical row must match `key`.
    pub fn within_group<E: Expression>(self, key: E) -> Agg<T, NotNull, (S, E::Scope)>
    where
        A: Arg<E::Sql>,
    {
        self.order(key)
    }
}

/// `percentile_cont(fraction) WITHIN GROUP (...)`
pub fn percentile_cont<F: Arg<Float8>>(fraction: F) -> WithinGroup<Continuous, (), F::Scope> {
    within_group("percentile_cont", vec![fraction.into_arg()])
}

/// `percentile_disc(fraction) WITHIN GROUP (...)`
pub fn percentile_disc<F: Arg<Float8>>(fraction: F) -> WithinGroup<Discrete, (), F::Scope> {
    within_group("percentile_disc", vec![fraction.into_arg()])
}

/// `mode() WITHIN GROUP (...)`
pub fn mode() -> WithinGroup<Discrete, (), ()> {
    within_group("mode", Vec::new())
}

/// Hypothetical-set aggregates: `rank(value) WITHIN GROUP (ORDER BY key)`.
pub mod hypothetical {
    use super::*;

    /// `rank(value) WITHIN GROUP (...)`
    pub fn rank<E: Expression>(value: E) -> WithinGroup<Hypothetical<Int8>, E, E::Scope> {
        within_group("rank", vec![value.into_node()])
    }

    /// `dense_rank(value) WITHIN GROUP (...)`
    pub fn dense_rank<E: Expression>(value: E) -> WithinGroup<Hypothetical<Int8>, E, E::Scope> {
        within_group("dense_rank", vec![value.into_node()])
    }

    /// `percent_rank(value) WITHIN GROUP (...)`
    pub fn percent_rank<E: Expression>(value: E) -> WithinGroup<Hypothetical<Float8>, E, E::Scope> {
        within_group("percent_rank", vec![value.into_node()])
    }

    /// `cume_dist(value) WITHIN GROUP (...)`
    pub fn cume_dist<E: Expression>(value: E) -> WithinGroup<Hypothetical<Float8>, E, E::Scope> {
        within_group("cume_dist", vec![value.into_node()])
    }
}

// ---------------------------------------------------------------------------
// Window functions

/// A window function call awaiting `OVER`.
pub struct WindowCall<T, N, S> {
    call: Call,
    _type: PhantomData<fn() -> (T, N, S)>,
}

fn window_call<T, N, S>(name: &'static str, args: Vec<Node>) -> WindowCall<T, N, S> {
    WindowCall {
        call: Call {
            name,
            args,
            ..Call::default()
        },
        _type: PhantomData,
    }
}

impl<T, N, S> WindowCall<T, N, S> {
    /// `f(...) OVER window`
    pub fn over<W: IntoWindow>(mut self, window: W) -> Expr<T, N, (S, W::Scope)> {
        self.call.over = Some(window.into_window());
        Expr::new(Node::Call(Box::new(self.call)))
    }

    /// The offset argument of `lag`/`lead`.
    pub fn offset<O: Arg<Int4>>(mut self, offset: O) -> WindowCall<T, N, (S, O::Scope)> {
        self.call.args.push(offset.into_arg());
        WindowCall {
            call: self.call,
            _type: PhantomData,
        }
    }

    /// The default argument of `lag`/`lead`, after the offset.
    pub fn default<D: Arg<T>>(mut self, default: D) -> WindowCall<T, N, (S, D::Scope)> {
        self.call.args.push(default.into_arg());
        WindowCall {
            call: self.call,
            _type: PhantomData,
        }
    }
}

/// `row_number()`
pub fn row_number() -> WindowCall<Int8, NotNull, ()> {
    window_call("row_number", Vec::new())
}

/// `rank()`
pub fn rank() -> WindowCall<Int8, NotNull, ()> {
    window_call("rank", Vec::new())
}

/// `dense_rank()`
pub fn dense_rank() -> WindowCall<Int8, NotNull, ()> {
    window_call("dense_rank", Vec::new())
}

/// `percent_rank()`
pub fn percent_rank() -> WindowCall<Float8, NotNull, ()> {
    window_call("percent_rank", Vec::new())
}

/// `cume_dist()`
pub fn cume_dist() -> WindowCall<Float8, NotNull, ()> {
    window_call("cume_dist", Vec::new())
}

/// `ntile(buckets)`
pub fn ntile<B: Arg<Int4>>(buckets: B) -> WindowCall<Int4, Nullable, B::Scope> {
    window_call("ntile", vec![buckets.into_arg()])
}

/// `lag(value)`
pub fn lag<E: Expression>(value: E) -> WindowCall<E::Sql, Nullable, E::Scope> {
    window_call("lag", vec![value.into_node()])
}

/// `lead(value)`
pub fn lead<E: Expression>(value: E) -> WindowCall<E::Sql, Nullable, E::Scope> {
    window_call("lead", vec![value.into_node()])
}

/// `first_value(value)`
pub fn first_value<E: Expression>(value: E) -> WindowCall<E::Sql, Nullable, E::Scope> {
    window_call("first_value", vec![value.into_node()])
}

/// `last_value(value)`
pub fn last_value<E: Expression>(value: E) -> WindowCall<E::Sql, Nullable, E::Scope> {
    window_call("last_value", vec![value.into_node()])
}

/// `nth_value(value, n)`
pub fn nth_value<E: Expression, I: Arg<Int4>>(
    value: E,
    n: I,
) -> WindowCall<E::Sql, Nullable, (E::Scope, I::Scope)> {
    window_call("nth_value", vec![value.into_node(), n.into_arg()])
}

/// A window definition.
pub struct WindowSpec<S> {
    window: Window,
    _type: PhantomData<fn() -> S>,
}

impl<S> Clone for WindowSpec<S> {
    fn clone(&self) -> Self {
        WindowSpec {
            window: self.window.clone(),
            _type: PhantomData,
        }
    }
}

/// An empty window definition, `OVER ()`.
pub fn window() -> WindowSpec<()> {
    WindowSpec {
        window: Window::default(),
        _type: PhantomData,
    }
}

impl<S> WindowSpec<S> {
    fn retype<S2>(self) -> WindowSpec<S2> {
        WindowSpec {
            window: self.window,
            _type: PhantomData,
        }
    }

    fn frame<A, B>(
        mut self,
        mode: &'static str,
        start: FrameBound<A>,
        end: FrameBound<B>,
    ) -> WindowSpec<(S, (A, B))> {
        self.window.frame = vec![
            Node::Keyword(mode),
            Node::Keyword("BETWEEN"),
            start.0,
            Node::Keyword("AND"),
            end.0,
        ];
        self.retype()
    }

    /// `PARTITION BY keys`
    pub fn partition_by<E: Exprs>(mut self, keys: E) -> WindowSpec<(S, E::Scope)> {
        keys.push_nodes(&mut self.window.partition);
        self.retype()
    }

    /// `ORDER BY keys`
    pub fn order_by<O: OrderBy>(mut self, keys: O) -> WindowSpec<(S, O::Scope)> {
        keys.push_keys(&mut self.window.order);
        self.retype()
    }

    /// `ROWS BETWEEN start AND end`
    pub fn rows_between<A, B>(
        self,
        start: FrameBound<A>,
        end: FrameBound<B>,
    ) -> WindowSpec<(S, (A, B))> {
        self.frame("ROWS", start, end)
    }

    /// `RANGE BETWEEN start AND end`
    pub fn range_between<A, B>(
        self,
        start: FrameBound<A>,
        end: FrameBound<B>,
    ) -> WindowSpec<(S, (A, B))> {
        self.frame("RANGE", start, end)
    }

    /// `GROUPS BETWEEN start AND end`
    pub fn groups_between<A, B>(
        self,
        start: FrameBound<A>,
        end: FrameBound<B>,
    ) -> WindowSpec<(S, (A, B))> {
        self.frame("GROUPS", start, end)
    }

    fn exclude(mut self, what: &'static str) -> Self {
        self.window.frame.push(Node::Keyword(what));
        self
    }

    /// `EXCLUDE CURRENT ROW`
    pub fn exclude_current_row(self) -> Self {
        self.exclude("EXCLUDE CURRENT ROW")
    }

    /// `EXCLUDE GROUP`
    pub fn exclude_group(self) -> Self {
        self.exclude("EXCLUDE GROUP")
    }

    /// `EXCLUDE TIES`
    pub fn exclude_ties(self) -> Self {
        self.exclude("EXCLUDE TIES")
    }
}

/// A frame boundary.
pub struct FrameBound<S>(Node, PhantomData<fn() -> S>);

fn bound<S>(parts: Vec<Node>) -> FrameBound<S> {
    FrameBound(Node::Seq(parts), PhantomData)
}

/// `UNBOUNDED PRECEDING`
pub fn unbounded_preceding() -> FrameBound<()> {
    bound(vec![Node::Keyword("UNBOUNDED PRECEDING")])
}

/// `offset PRECEDING`
pub fn preceding<E: Expression>(offset: E) -> FrameBound<E::Scope> {
    bound(vec![offset.into_node(), Node::Keyword("PRECEDING")])
}

/// `CURRENT ROW`
pub fn current_row() -> FrameBound<()> {
    bound(vec![Node::Keyword("CURRENT ROW")])
}

/// `offset FOLLOWING`
pub fn following<E: Expression>(offset: E) -> FrameBound<E::Scope> {
    bound(vec![offset.into_node(), Node::Keyword("FOLLOWING")])
}

/// `UNBOUNDED FOLLOWING`
pub fn unbounded_following() -> FrameBound<()> {
    bound(vec![Node::Keyword("UNBOUNDED FOLLOWING")])
}

/// A window defined in a `WINDOW` clause; add it to the query with
/// [`Select::window`].
pub struct NamedWindow<S> {
    pub(crate) name: &'static str,
    pub(crate) spec: Window,
    _type: PhantomData<fn() -> S>,
}

/// `WINDOW name AS (spec)`
pub fn named_window<S>(name: &'static str, spec: WindowSpec<S>) -> NamedWindow<S> {
    NamedWindow {
        name,
        spec: spec.window,
        _type: PhantomData,
    }
}

impl<S> NamedWindow<S> {
    fn based(&self) -> WindowSpec<()> {
        WindowSpec {
            window: Window {
                base: Some(self.name),
                ..Window::default()
            },
            _type: PhantomData,
        }
    }

    /// `(name ROWS BETWEEN start AND end)`
    pub fn rows_between<A, B>(
        &self,
        start: FrameBound<A>,
        end: FrameBound<B>,
    ) -> WindowSpec<((), (A, B))> {
        self.based().rows_between(start, end)
    }

    /// `(name RANGE BETWEEN start AND end)`
    pub fn range_between<A, B>(
        &self,
        start: FrameBound<A>,
        end: FrameBound<B>,
    ) -> WindowSpec<((), (A, B))> {
        self.based().range_between(start, end)
    }
}

/// A window for `OVER`.
pub trait IntoWindow {
    type Scope;
    fn into_window(self) -> Window;
}

impl<S> IntoWindow for WindowSpec<S> {
    type Scope = S;
    fn into_window(self) -> Window {
        self.window
    }
}

impl<S> IntoWindow for &NamedWindow<S> {
    type Scope = ();
    fn into_window(self) -> Window {
        Window {
            base: Some(self.name),
            ..Window::default()
        }
    }
}

// ---------------------------------------------------------------------------
// Conditional expressions

/// `coalesce(first, second)`
pub fn coalesce<A: Expression, B: Arg<A::Sql>>(
    first: A,
    second: B,
) -> Expr<A::Sql, <A::Null as Nullability>::And<B::Null>, (A::Scope, B::Scope)> {
    call("coalesce", vec![first.into_node(), second.into_arg()])
}

/// `nullif(value, other)`
pub fn nullif<A: Expression, B: Arg<A::Sql>>(
    value: A,
    other: B,
) -> Expr<A::Sql, Nullable, (A::Scope, B::Scope)> {
    call("nullif", vec![value.into_node(), other.into_arg()])
}

/// `greatest(first, second)`
pub fn greatest<A: Expression, B: Arg<A::Sql>>(
    first: A,
    second: B,
) -> Expr<A::Sql, <A::Null as Nullability>::And<B::Null>, (A::Scope, B::Scope)> {
    call("greatest", vec![first.into_node(), second.into_arg()])
}

/// `least(first, second)`
pub fn least<A: Expression, B: Arg<A::Sql>>(
    first: A,
    second: B,
) -> Expr<A::Sql, <A::Null as Nullability>::And<B::Null>, (A::Scope, B::Scope)> {
    call("least", vec![first.into_node(), second.into_arg()])
}

// ---------------------------------------------------------------------------
// String, math and date/time functions

/// Types with `lower`/`upper`: text, and range bounds.
pub trait LowerUpper {
    type Out: SqlType;
    type Null: Nullability;
}
impl LowerUpper for Text {
    type Out = Text;
    type Null = NotNull;
}
impl LowerUpper for Varchar {
    type Out = Text;
    type Null = NotNull;
}
impl LowerUpper for Bpchar {
    type Out = Text;
    type Null = NotNull;
}
impl<T: RangeElement> LowerUpper for Range<T> {
    type Out = T;
    type Null = Nullable;
}
impl<T: RangeElement> LowerUpper for Multirange<T> {
    type Out = T;
    type Null = Nullable;
}

/// Result of `lower`/`upper`.
pub type Bound<E> = Expr<
    <<E as Expression>::Sql as LowerUpper>::Out,
    Or<<E as Expression>::Null, <<E as Expression>::Sql as LowerUpper>::Null>,
    <E as Expression>::Scope,
>;

/// `lower(text)` or the lower bound of a range.
pub fn lower<E: Expression>(value: E) -> Bound<E>
where
    E::Sql: LowerUpper,
{
    call("lower", vec![value.into_node()])
}

/// `upper(text)` or the upper bound of a range.
pub fn upper<E: Expression>(value: E) -> Bound<E>
where
    E::Sql: LowerUpper,
{
    call("upper", vec![value.into_node()])
}

/// Types with `length`.
pub trait HasLength {}
impl HasLength for Text {}
impl HasLength for Varchar {}
impl HasLength for Bpchar {}
impl HasLength for Bytea {}
impl HasLength for Bit {}
impl HasLength for Varbit {}
impl HasLength for Tsvector {}

/// `length(value)`
pub fn length<E: Expression>(value: E) -> Expr<Int4, E::Null, E::Scope>
where
    E::Sql: HasLength,
{
    call("length", vec![value.into_node()])
}

/// `substr(text, start, count)`
pub fn substr<E: Arg<Text>, A: Arg<Int4>, B: Arg<Int4>>(
    text: E,
    start: A,
    count: B,
) -> Expr<Text, Or<Or<E::Null, A::Null>, B::Null>, (E::Scope, (A::Scope, B::Scope))> {
    call(
        "substr",
        vec![text.into_arg(), start.into_arg(), count.into_arg()],
    )
}

/// `concat_ws(separator, values...)`; NULL values are skipped.
pub fn concat_ws<D: Arg<Text>, E: Exprs>(
    separator: D,
    values: E,
) -> Expr<Text, D::Null, (D::Scope, E::Scope)> {
    let mut args = vec![separator.into_arg()];
    values.push_nodes(&mut args);
    call("concat_ws", args)
}

/// `format(format, values...)`
pub fn format<F: Arg<Text>, E: Exprs>(
    format: F,
    values: E,
) -> Expr<Text, F::Null, (F::Scope, E::Scope)> {
    let mut args = vec![format.into_arg()];
    values.push_nodes(&mut args);
    call("format", args)
}

/// `string_to_array(text, delimiter)`
pub fn string_to_array<E: Arg<Text>, D: Arg<Text>>(
    text: E,
    delimiter: D,
) -> Expr<Array<Text>, Or<E::Null, D::Null>, (E::Scope, D::Scope)> {
    call(
        "string_to_array",
        vec![text.into_arg(), delimiter.into_arg()],
    )
}

/// Numeric types.
pub trait Number: SqlType {
    /// The type `power` computes in.
    type Power: SqlType;
}
impl Number for Int2 {
    type Power = Float8;
}
impl Number for Int4 {
    type Power = Float8;
}
impl Number for Int8 {
    type Power = Float8;
}
impl Number for Float4 {
    type Power = Float8;
}
impl Number for Float8 {
    type Power = Float8;
}
impl Number for Numeric {
    type Power = Numeric;
}

/// `abs(value)`
pub fn abs<E: Expression>(value: E) -> Expr<E::Sql, E::Null, E::Scope>
where
    E::Sql: Number,
{
    call("abs", vec![value.into_node()])
}

/// `round(value, digits)`
pub fn round<E: Arg<Numeric>, D: Arg<Int4>>(
    value: E,
    digits: D,
) -> Expr<Numeric, Or<E::Null, D::Null>, (E::Scope, D::Scope)> {
    call("round", vec![value.into_arg(), digits.into_arg()])
}

/// `power(base, exponent)`
pub fn power<E: Expression, X: Arg<<E::Sql as Number>::Power>>(
    base: E,
    exponent: X,
) -> Expr<<E::Sql as Number>::Power, Or<E::Null, X::Null>, (E::Scope, X::Scope)>
where
    E::Sql: Number,
{
    call("power", vec![base.into_node(), exponent.into_arg()])
}

/// `CURRENT_DATE`
pub fn current_date() -> Expr<Date> {
    Expr::new(Node::Keyword("CURRENT_DATE"))
}

/// `make_date(year => .., month => .., day => ..)`
pub fn make_date<Y: Arg<Int4>, M: Arg<Int4>, D: Arg<Int4>>(
    year: Y,
    month: M,
    day: D,
) -> Expr<Date, Or<Or<Y::Null, M::Null>, D::Null>, (Y::Scope, (M::Scope, D::Scope))> {
    call(
        "make_date",
        vec![
            named("year", year.into_arg()),
            named("month", month.into_arg()),
            named("day", day.into_arg()),
        ],
    )
}

/// `make_time(hour, min, sec)`
pub fn make_time<H: Arg<Int4>, M: Arg<Int4>, S: Arg<Float8>>(
    hour: H,
    min: M,
    sec: S,
) -> Expr<Time, Or<Or<H::Null, M::Null>, S::Null>, (H::Scope, (M::Scope, S::Scope))> {
    call(
        "make_time",
        vec![hour.into_arg(), min.into_arg(), sec.into_arg()],
    )
}

fn named(name: &'static str, value: Node) -> Node {
    Node::Seq(vec![Node::Keyword(name), Node::Keyword("=>"), value])
}

/// `make_interval(...)` with named arguments.
pub struct MakeInterval<S> {
    args: Vec<Node>,
    _type: PhantomData<fn() -> S>,
}

/// `make_interval()`; add components with named arguments.
pub fn make_interval() -> MakeInterval<()> {
    MakeInterval {
        args: Vec::new(),
        _type: PhantomData,
    }
}

macro_rules! interval_parts {
    ($($name:ident: $t:ty),*) => {
        impl<S> MakeInterval<S> {
            $(
                #[doc = concat!("`", stringify!($name), " => value`")]
                pub fn $name<V: Arg<$t>>(mut self, value: V) -> MakeInterval<(S, V::Scope)> {
                    self.args.push(named(stringify!($name), value.into_arg()));
                    MakeInterval { args: self.args, _type: PhantomData }
                }
            )*
        }
    };
}

interval_parts!(years: Int4, months: Int4, weeks: Int4, days: Int4, hours: Int4, mins: Int4, secs: Float8);

impl<S> Expression for MakeInterval<S> {
    type Sql = Interval;
    type Null = Nullable;
    type Scope = S;
    fn into_node(self) -> Node {
        Node::call("make_interval", self.args)
    }
}

/// Date and time types with `date_trunc`.
pub trait Truncatable: SqlType {}
impl Truncatable for Timestamp {}
impl Truncatable for Timestamptz {}
impl Truncatable for Interval {}

/// `date_trunc(field, value)`
pub fn date_trunc<F: Arg<Text>, E: Expression>(
    field: F,
    value: E,
) -> Expr<E::Sql, Or<F::Null, E::Null>, (F::Scope, E::Scope)>
where
    E::Sql: Truncatable,
{
    call("date_trunc", vec![field.into_arg(), value.into_node()])
}

/// Fields for `EXTRACT`.
#[derive(Clone, Copy, Debug)]
pub enum DateField {
    Century,
    Day,
    Decade,
    Dow,
    Doy,
    Epoch,
    Hour,
    Isodow,
    Isoyear,
    Microseconds,
    Millennium,
    Milliseconds,
    Minute,
    Month,
    Quarter,
    Second,
    Timezone,
    Week,
    Year,
}

impl DateField {
    fn keyword(self) -> &'static str {
        use DateField::*;
        match self {
            Century => "CENTURY",
            Day => "DAY",
            Decade => "DECADE",
            Dow => "DOW",
            Doy => "DOY",
            Epoch => "EPOCH",
            Hour => "HOUR",
            Isodow => "ISODOW",
            Isoyear => "ISOYEAR",
            Microseconds => "MICROSECONDS",
            Millennium => "MILLENNIUM",
            Milliseconds => "MILLISECONDS",
            Minute => "MINUTE",
            Month => "MONTH",
            Quarter => "QUARTER",
            Second => "SECOND",
            Timezone => "TIMEZONE",
            Week => "WEEK",
            Year => "YEAR",
        }
    }
}

/// Date and time types.
pub trait DateTime: SqlType {}
impl DateTime for Date {}
impl DateTime for Time {}
impl DateTime for Timetz {}
impl DateTime for Timestamp {}
impl DateTime for Timestamptz {}
impl DateTime for Interval {}

/// `EXTRACT(field FROM value)`
pub fn extract<E: Expression>(field: DateField, value: E) -> Expr<Numeric, E::Null, E::Scope>
where
    E::Sql: DateTime,
{
    call(
        "EXTRACT",
        vec![Node::Seq(vec![
            Node::Keyword(field.keyword()),
            Node::Keyword("FROM"),
            value.into_node(),
        ])],
    )
}

/// Types formatted by `to_char`.
pub trait Formattable: SqlType {}
impl Formattable for Date {}
impl Formattable for Timestamp {}
impl Formattable for Timestamptz {}
impl Formattable for Interval {}
impl Formattable for Int4 {}
impl Formattable for Int8 {}
impl Formattable for Numeric {}
impl Formattable for Float4 {}
impl Formattable for Float8 {}

/// `to_char(value, format)`
pub fn to_char<E: Expression, F: Arg<Text>>(
    value: E,
    format: F,
) -> Expr<Text, Or<E::Null, F::Null>, (E::Scope, F::Scope)>
where
    E::Sql: Formattable,
{
    call("to_char", vec![value.into_node(), format.into_arg()])
}

/// `pg_typeof(value)`
pub fn pg_typeof<E: Expression>(value: E) -> Expr<Regtype, NotNull, E::Scope> {
    call("pg_typeof", vec![value.into_node()])
}

/// `merge_action()` in `MERGE ... RETURNING`.
pub fn merge_action() -> Expr<Text> {
    call("merge_action", Vec::new())
}

// ---------------------------------------------------------------------------
// Rows and arrays

/// `ROW(...)`: a row value with fields `R`.
pub struct RowExpr<R, S> {
    values: Vec<Node>,
    _type: PhantomData<fn() -> (R, S)>,
}

/// `ROW(values...)`
pub fn row<V: ValuesRow>(values: V) -> RowExpr<V::Row, V::Scope> {
    RowExpr {
        values: values.into_nodes(),
        _type: PhantomData,
    }
}

impl<R: 'static, S> Expression for RowExpr<R, S> {
    type Sql = Record<R>;
    type Null = NotNull;
    type Scope = S;
    fn into_node(self) -> Node {
        Node::call("ROW", self.values)
    }
}

impl<R: 'static, S> RowExpr<R, S> {
    fn compare<R2: 'static, S2>(
        self,
        op: &'static str,
        other: RowExpr<R2, S2>,
    ) -> Expr<Bool, Nullable, (S, S2)>
    where
        R: Unify<R2>,
    {
        Expr::new(Node::binary(
            op,
            Expression::into_node(self),
            Expression::into_node(other),
        ))
    }

    /// Row-wise `self = other`
    pub fn eq<R2: 'static, S2>(self, other: RowExpr<R2, S2>) -> Expr<Bool, Nullable, (S, S2)>
    where
        R: Unify<R2>,
    {
        self.compare("=", other)
    }

    /// Row-wise `self < other`
    pub fn lt<R2: 'static, S2>(self, other: RowExpr<R2, S2>) -> Expr<Bool, Nullable, (S, S2)>
    where
        R: Unify<R2>,
    {
        self.compare("<", other)
    }

    /// Row-wise `self > other`
    pub fn gt<R2: 'static, S2>(self, other: RowExpr<R2, S2>) -> Expr<Bool, Nullable, (S, S2)>
    where
        R: Unify<R2>,
    {
        self.compare(">", other)
    }

    /// `(values...) IN (subquery)` with a matching row type.
    pub fn in_<Q: Statement>(self, query: Q) -> Expr<Bool, Nullable, (S, Q::Scope)>
    where
        R: Unify<Q::Row>,
    {
        let values = Node::List(self.values);
        Expr::new(Node::binary(
            "IN",
            values,
            Node::Subquery(Box::new(query.into_stmt())),
        ))
    }
}

/// `ARRAY[values...]`; all values have the type of the first.
pub fn array<V: ArrayItems>(values: V) -> Expr<Array<V::Sql>, NotNull, V::Scope> {
    Expr::new(Node::Array(values.into_nodes()))
}

/// Elements of `ARRAY[...]`.
pub trait ArrayItems {
    type Sql: SqlType;
    type Scope;
    fn into_nodes(self) -> Vec<Node>;
}

macro_rules! array_items {
    ($(($first:ident $($t:ident $i:tt),*))*) => {$(
        impl<$first: Expression, $($t: Arg<$first::Sql>),*> ArrayItems for ($first, $($t,)*) {
            type Sql = $first::Sql;
            type Scope = array_items!(@scope $first::Scope $(, $t::Scope)*);
            fn into_nodes(self) -> Vec<Node> {
                vec![self.0.into_node(), $(self.$i.into_arg()),*]
            }
        }
    )*};
    (@scope $a:ty) => { $a };
    (@scope $a:ty, $($rest:ty),+) => { ($a, array_items!(@scope $($rest),+)) };
}

array_items! { (A) (A B 1) (A B 1, C 2) (A B 1, C 2, D 3) (A B 1, C 2, D 3, E 4) (A B 1, C 2, D 3, E 4, F 5) }

/// `cardinality(array)`
pub fn cardinality<E: Expression>(array: E) -> Expr<Int4, E::Null, E::Scope>
where
    E::Sql: Element,
{
    call("cardinality", vec![array.into_node()])
}

/// Field access on composite values, used by generated composite types.
#[doc(hidden)]
pub fn field<E: Expression, T>(value: E, name: &'static str) -> Expr<T, Nullable, E::Scope> {
    Expr::new(Node::Field(Box::new(value.into_node()), name))
}

/// `(composite).*` in a `SELECT` list.
pub fn expand<E: Expression>(value: E) -> Star<<E::Sql as Composite>::Row, E::Scope>
where
    E::Sql: Composite,
{
    Star::new(Node::Field(Box::new(value.into_node()), "*"))
}

// ---------------------------------------------------------------------------
// Ranges and enums

/// Range constructor call, e.g. `int4range(low, high)`.
pub struct RangeCall<T, N, S> {
    name: &'static str,
    args: Vec<Node>,
    _type: PhantomData<fn() -> (T, N, S)>,
}

impl<T, N, S> RangeCall<T, N, S> {
    /// The bounds argument, such as `"[]"`.
    pub fn bounds<B: Arg<Text>>(mut self, bounds: B) -> RangeCall<T, N, (S, B::Scope)> {
        self.args.push(bounds.into_arg());
        RangeCall {
            name: self.name,
            args: self.args,
            _type: PhantomData,
        }
    }
}

impl<T: SqlType, N: Nullability, S> Expression for RangeCall<T, N, S> {
    type Sql = T;
    type Null = N;
    type Scope = S;
    fn into_node(self) -> Node {
        Node::call(self.name, self.args)
    }
}

/// `int4range(low, high)`; NULL bounds are unbounded.
pub fn int4range<A: Arg<Int4>, B: Arg<Int4>>(
    low: A,
    high: B,
) -> RangeCall<Range<Int4>, NotNull, (A::Scope, B::Scope)> {
    RangeCall {
        name: "int4range",
        args: vec![low.into_arg(), high.into_arg()],
        _type: PhantomData,
    }
}

/// `int4multirange(ranges...)`
pub fn int4multirange<E: Arg<Range<Int4>>>(range: E) -> Expr<Multirange<Int4>, E::Null, E::Scope> {
    call("int4multirange", vec![range.into_arg()])
}

/// `isempty(range)`
pub fn isempty<E: Expression>(range: E) -> Expr<Bool, E::Null, E::Scope>
where
    E::Sql: RangeLike,
{
    call("isempty", vec![range.into_node()])
}

/// `lower_inc(range)`
pub fn lower_inc<E: Expression>(range: E) -> Expr<Bool, E::Null, E::Scope>
where
    E::Sql: RangeLike,
{
    call("lower_inc", vec![range.into_node()])
}

/// `upper_inc(range)`
pub fn upper_inc<E: Expression>(range: E) -> Expr<Bool, E::Null, E::Scope>
where
    E::Sql: RangeLike,
{
    call("upper_inc", vec![range.into_node()])
}

/// `enum_first(value)`
pub fn enum_first<E: Expression>(value: E) -> Expr<E::Sql, NotNull, E::Scope>
where
    E::Sql: Enum,
{
    call("enum_first", vec![value.into_node()])
}

/// `enum_last(value)`
pub fn enum_last<E: Expression>(value: E) -> Expr<E::Sql, NotNull, E::Scope>
where
    E::Sql: Enum,
{
    call("enum_last", vec![value.into_node()])
}

/// `enum_range(NULL::E)`: all values of enum type `E`.
pub fn enum_range<E: Enum>() -> Expr<Array<E>> {
    call(
        "enum_range",
        vec![Node::Cast(
            Box::new(Node::Keyword("NULL")),
            type_name::<E>(),
        )],
    )
}

/// `enum_range(low, high)`; NULL bounds are open.
pub fn enum_range_between<A: Expression, B: Arg<A::Sql>>(
    low: A,
    high: B,
) -> Expr<Array<A::Sql>, NotNull, (A::Scope, B::Scope)>
where
    A::Sql: Enum,
{
    call("enum_range", vec![low.into_node(), high.into_arg()])
}

// ---------------------------------------------------------------------------
// JSON

/// `jsonb_build_object(key, value, ...)`
pub fn jsonb_build_object<E: Exprs>(keys_and_values: E) -> Expr<Jsonb, NotNull, E::Scope> {
    call("jsonb_build_object", keys_and_values.into_nodes())
}

/// `jsonb_set(target, path, value)`
pub fn jsonb_set<E: Arg<Jsonb>, P: Arg<Array<Text>>, V: Arg<Jsonb>>(
    target: E,
    path: P,
    value: V,
) -> Expr<Jsonb, Or<Or<E::Null, P::Null>, V::Null>, (E::Scope, (P::Scope, V::Scope))> {
    call(
        "jsonb_set",
        vec![target.into_arg(), path.into_arg(), value.into_arg()],
    )
}

/// `jsonb_path_query_array(target, path)`
pub fn jsonb_path_query_array<E: Arg<Jsonb>, P: Arg<Jsonpath>>(
    target: E,
    path: P,
) -> Expr<Jsonb, Or<E::Null, P::Null>, (E::Scope, P::Scope)> {
    call(
        "jsonb_path_query_array",
        vec![target.into_arg(), path.into_arg()],
    )
}

/// `jsonb_path_exists(target, path, vars)`
pub fn jsonb_path_exists<E: Arg<Jsonb>, P: Arg<Jsonpath>, V: Arg<Jsonb>>(
    target: E,
    path: P,
    vars: V,
) -> Expr<Bool, Or<Or<E::Null, P::Null>, V::Null>, (E::Scope, (P::Scope, V::Scope))> {
    call(
        "jsonb_path_exists",
        vec![target.into_arg(), path.into_arg(), vars.into_arg()],
    )
}

/// Key-value pairs of `JSON_OBJECT`.
pub trait JsonPairs {
    type Scope;
    fn into_nodes(self) -> Vec<Node>;
}

macro_rules! json_pairs {
    ($(($($k:ident $v:ident $i:tt),+))*) => {$(
        impl<$($k: Arg<Text>, $v: Expression),+> JsonPairs for ($(($k, $v),)+) {
            type Scope = ($(($k::Scope, $v::Scope),)+);
            fn into_nodes(self) -> Vec<Node> {
                vec![$({
                    let (key, value) = self.$i;
                    Node::Seq(vec![key.into_arg(), Node::Keyword(":"), value.into_node()])
                }),+]
            }
        }
    )*};
}

json_pairs! { (K V 0) (K V 0, K2 V2 1) (K V 0, K2 V2 1, K3 V3 2) (K V 0, K2 V2 1, K3 V3 2, K4 V4 3) }

/// `JSON_OBJECT(key : value, ...)`
pub fn json_object<P: JsonPairs>(pairs: P) -> Expr<Json, NotNull, P::Scope> {
    call("JSON_OBJECT", pairs.into_nodes())
}

/// `JSON_ARRAY(values...)`
pub fn json_array<E: Exprs>(values: E) -> Expr<Json, NotNull, E::Scope> {
    call("JSON_ARRAY", values.into_nodes())
}

/// `JSON(text)`
pub fn json<E: Arg<Text>>(text: E) -> Expr<Json, E::Null, E::Scope> {
    call("JSON", vec![text.into_arg()])
}

/// `JSON_SCALAR(value)`
pub fn json_scalar<E: Expression>(value: E) -> Expr<Json, E::Null, E::Scope> {
    call("JSON_SCALAR", vec![value.into_node()])
}

/// `JSON_EXISTS(value, path)`
pub fn json_exists<E: Expression, P: Arg<Jsonpath>>(
    value: E,
    path: P,
) -> Expr<Bool, Nullable, (E::Scope, P::Scope)>
where
    E::Sql: JsonInput,
{
    call("JSON_EXISTS", vec![value.into_node(), path.into_arg()])
}

/// An SQL/JSON function call with trailing clauses, producing `T`.
pub struct JsonCall<T, S> {
    name: &'static str,
    args: Vec<Node>,
    clauses: Vec<Node>,
    _type: PhantomData<fn() -> (T, S)>,
}

impl<T, S> JsonCall<T, S> {
    fn new(name: &'static str, args: Vec<Node>) -> Self {
        JsonCall {
            name,
            args,
            clauses: Vec::new(),
            _type: PhantomData,
        }
    }

    fn clause<T2, S2>(mut self, parts: Vec<Node>) -> JsonCall<T2, S2> {
        self.clauses.extend(parts);
        JsonCall {
            name: self.name,
            args: self.args,
            clauses: self.clauses,
            _type: PhantomData,
        }
    }

    /// `RETURNING type`
    pub fn returning<R: CastTarget>(self) -> JsonCall<R::Sql, S> {
        let mut name = String::new();
        R::write_name(&mut name);
        self.clause(vec![Node::Keyword("RETURNING"), Node::Type(name)])
    }
}

impl<S> JsonCall<Jsonb, S> {
    /// `WITH WRAPPER`
    pub fn with_wrapper(self) -> Self {
        self.clause(vec![Node::Keyword("WITH WRAPPER")])
    }

    /// `EMPTY ARRAY ON EMPTY`
    pub fn empty_array_on_empty(self) -> Self {
        self.clause(vec![Node::Keyword("EMPTY ARRAY ON EMPTY")])
    }
}

impl<T: SqlType, S> JsonCall<T, S> {
    /// `DEFAULT value ON EMPTY`; the value is written as a literal.
    pub fn default_on_empty<V: Bind>(self, value: V) -> Self
    where
        V::Sql: Coerce<T>,
    {
        let value = literal(value).into_node();
        self.clause(vec![
            Node::Keyword("DEFAULT"),
            value,
            Node::Keyword("ON EMPTY"),
        ])
    }

    /// `DEFAULT value ON ERROR`; the value is written as a literal.
    pub fn default_on_error<V: Bind>(self, value: V) -> Self
    where
        V::Sql: Coerce<T>,
    {
        let value = literal(value).into_node();
        self.clause(vec![
            Node::Keyword("DEFAULT"),
            value,
            Node::Keyword("ON ERROR"),
        ])
    }
}

impl<T: SqlType, S> Expression for JsonCall<T, S> {
    type Sql = T;
    type Null = Nullable;
    type Scope = S;
    fn into_node(mut self) -> Node {
        if let Some(last) = self.args.pop() {
            let mut parts = vec![last];
            parts.extend(self.clauses);
            self.args.push(Node::Seq(parts));
        }
        Node::call(self.name, self.args)
    }
}

/// `JSON_QUERY(value, path)`
pub fn json_query<E: Expression, P: Arg<Jsonpath>>(
    value: E,
    path: P,
) -> JsonCall<Jsonb, (E::Scope, P::Scope)>
where
    E::Sql: JsonInput,
{
    JsonCall::new("JSON_QUERY", vec![value.into_node(), path.into_arg()])
}

/// `JSON_VALUE(value, path)`, returning `text` unless `returning` is used.
pub fn json_value<E: Expression, P: Arg<Jsonpath>>(
    value: E,
    path: P,
) -> JsonCall<Text, (E::Scope, P::Scope)>
where
    E::Sql: JsonInput,
{
    JsonCall::new("JSON_VALUE", vec![value.into_node(), path.into_arg()])
}

/// `JSON_SERIALIZE(value)`, returning `text` unless `returning` is used.
pub fn json_serialize<E: Expression>(value: E) -> JsonCall<Text, E::Scope>
where
    E::Sql: JsonInput,
{
    JsonCall::new("JSON_SERIALIZE", vec![value.into_node()])
}

// ---------------------------------------------------------------------------
// Set-returning functions

/// Identity of an unaliased set-returning function producing rows `R`.
pub struct Func<R>(PhantomData<R>);

impl<R: 'static> Qualifier for Func<R> {
    type Null = NotNull;
}

/// A set-returning function: usable in `FROM`, and in a `SELECT` list when
/// it returns a single column.
pub type SetFunction<R, S> = Rel<Func<R>, R, S>;

fn set_function<R, S>(
    name: &'static str,
    node: Node,
    columns: Vec<&'static str>,
) -> SetFunction<R, S> {
    let mut item = FromItem::new(FromKind::Function(node));
    item.alias = Some(name);
    item.columns = columns.clone();
    Rel::new(item, name, columns)
}

impl<X: Field, S> Expression for SetFunction<(X,), S> {
    type Sql = X::Sql;
    type Null = X::Null;
    type Scope = S;
    fn into_node(self) -> Node {
        match self.item.kind {
            FromKind::Function(node) => node,
            _ => unreachable!("set functions are function calls"),
        }
    }
}

/// Appending a field to a row.
pub trait Append<X> {
    type Out;
}

macro_rules! append {
    ($(($($t:ident),*))*) => {$(
        impl<X, $($t),*> Append<X> for ($($t,)*) {
            type Out = ($($t,)* X,);
        }
    )*};
}

append! { (A) (A, B) (A, B, C) (A, B, C, D) (A, B, C, D, E) }

impl<R: Append<Int8>, S> SetFunction<R, S> {
    /// `WITH ORDINALITY`: adds an `ordinality` column.
    pub fn with_ordinality(mut self) -> SetFunction<R::Out, S> {
        if let FromKind::Function(node) = self.item.kind {
            self.item.kind =
                FromKind::Function(Node::Seq(vec![node, Node::Keyword("WITH ORDINALITY")]));
        }
        self.item.columns.push("ordinality");
        self.names.push("ordinality");
        Rel::new(self.item, self.qualifier, self.names)
    }
}

/// Types of `generate_series` bounds.
pub trait Series: SqlType {}
impl Series for Int4 {}
impl Series for Int8 {}
impl Series for Numeric {}

/// `generate_series(start, stop)`
pub fn generate_series<A: Expression, B: Arg<A::Sql>>(
    start: A,
    stop: B,
) -> SetFunction<(A::Sql,), (A::Scope, B::Scope)>
where
    A::Sql: Series,
{
    let node = Node::call("generate_series", vec![start.into_node(), stop.into_arg()]);
    set_function("generate_series", node, vec!["generate_series"])
}

/// Types expanded by `unnest`.
pub trait Unnest {
    type Item: SqlType;
}
impl<T: SqlType> Unnest for Array<T> {
    type Item = T;
}
impl<T: RangeElement> Unnest for Multirange<T> {
    type Item = Range<T>;
}

/// `unnest(array)` or `unnest(multirange)`
pub fn unnest<E: Expression>(
    values: E,
) -> SetFunction<(Option<<E::Sql as Unnest>::Item>,), E::Scope>
where
    E::Sql: Unnest,
{
    set_function(
        "unnest",
        Node::call("unnest", vec![values.into_node()]),
        vec!["unnest"],
    )
}

/// `jsonb_path_query(target, path)`
pub fn jsonb_path_query<E: Arg<Jsonb>, P: Arg<Jsonpath>>(
    target: E,
    path: P,
) -> SetFunction<(Jsonb,), (E::Scope, P::Scope)> {
    let node = Node::call("jsonb_path_query", vec![target.into_arg(), path.into_arg()]);
    set_function("jsonb_path_query", node, vec!["jsonb_path_query"])
}

/// Single-column set functions combined by `ROWS FROM`.
pub trait RowsFrom {
    type Row;
    type Scope;
    fn into_parts(self) -> (Vec<Node>, Vec<&'static str>);
}

macro_rules! rows_from {
    ($(($($t:ident $s:ident $i:tt),+))*) => {$(
        impl<$($t: Field, $s),+> RowsFrom for ($(SetFunction<($t,), $s>,)+) {
            type Row = ($(Option<$t::Sql>,)+);
            type Scope = ($($s,)+);
            fn into_parts(self) -> (Vec<Node>, Vec<&'static str>) {
                let mut nodes = Vec::new();
                let mut names = Vec::new();
                $(
                    names.extend(self.$i.names.iter().copied());
                    if let FromKind::Function(node) = self.$i.item.kind {
                        nodes.push(node);
                    }
                )+
                (nodes, names)
            }
        }
    )*};
}

rows_from! { (A S 0, B T 1) (A S 0, B T 1, C U 2) (A S 0, B T 1, C U 2, D V 3) }

/// `ROWS FROM (functions...)`
pub fn rows_from<F: RowsFrom>(functions: F) -> SetFunction<F::Row, F::Scope> {
    let (nodes, names) = functions.into_parts();
    let node = Node::Seq(vec![Node::Keyword("ROWS FROM"), Node::List(nodes)]);
    set_function("rows_from", node, names)
}

/// A column of `JSON_TABLE`.
pub struct JsonColumn<T> {
    name: &'static str,
    path: &'static str,
    _type: PhantomData<fn() -> T>,
}

/// `name type PATH 'path'` in `JSON_TABLE ... COLUMNS (...)`.
pub fn json_column<T: SqlType>(name: &'static str, path: &'static str) -> JsonColumn<T> {
    JsonColumn {
        name,
        path,
        _type: PhantomData,
    }
}

/// Columns of `JSON_TABLE`.
pub trait JsonColumns {
    type Row;
    fn into_parts(self) -> (Vec<Node>, Vec<&'static str>);
}

macro_rules! json_columns {
    ($(($($t:ident $i:tt),+))*) => {$(
        impl<$($t: SqlType),+> JsonColumns for ($(JsonColumn<$t>,)+) {
            type Row = ($(Option<$t>,)+);
            fn into_parts(self) -> (Vec<Node>, Vec<&'static str>) {
                let columns = vec![$(Node::Seq(vec![
                    Node::Ident(self.$i.name),
                    Node::Type(type_name::<$t>()),
                    Node::Keyword("PATH"),
                    Node::Literal(self.$i.path.to_string(), String::new()),
                ])),+];
                (columns, vec![$(self.$i.name),+])
            }
        }
    )*};
}

json_columns! { (A 0) (A 0, B 1) (A 0, B 1, C 2) (A 0, B 1, C 2, D 3) }

/// `JSON_TABLE(value, 'path' COLUMNS (...))`; paths are written as literals.
pub fn json_table<E: Expression, C: JsonColumns>(
    value: E,
    path: &'static str,
    columns: C,
) -> SetFunction<C::Row, E::Scope>
where
    E::Sql: JsonInput,
{
    let (columns, names) = columns.into_parts();
    let path = Node::Seq(vec![
        Node::Literal(path.to_string(), String::new()),
        Node::Keyword("COLUMNS"),
        Node::List(columns),
    ]);
    let node = Node::call("JSON_TABLE", vec![value.into_node(), path]);
    let mut table = set_function("json_table", node, names);
    table.item.columns.clear();
    table
}

impl<T: SqlType, N: Nullability, S> ExpressionMethods for Agg<T, N, S> {}
impl<S> ExpressionMethods for MakeInterval<S> {}
impl<T: SqlType, N: Nullability, S> ExpressionMethods for RangeCall<T, N, S> {}
impl<T: SqlType, S> ExpressionMethods for JsonCall<T, S> {}
impl<X: Field, S> ExpressionMethods for SetFunction<(X,), S> {}

impl<R: 'static, S> ExpressionMethods for RowExpr<R, S> {}

/// A time period `(start, end)` for `OVERLAPS`; `end` is a point in time
/// of the same type or an interval.
pub struct Period<T, S> {
    bounds: Vec<Node>,
    _type: PhantomData<fn() -> (T, S)>,
}

/// `(start, end)` for `OVERLAPS`.
pub fn period<A: Expression, B: PeriodEnd<A::Sql>>(
    start: A,
    end: B,
) -> Period<A::Sql, (A::Scope, B::Scope)>
where
    A::Sql: DateTime,
{
    Period {
        bounds: vec![start.into_node(), end.into_node()],
        _type: PhantomData,
    }
}

/// The end of a period starting at type `T`.
pub trait PeriodEnd<T>: Expression {}
impl<T, E: Expression> PeriodEnd<T> for E where E::Sql: PeriodEndType<T> {}

/// Types that can end a period starting at `T`.
pub trait PeriodEndType<T> {}
impl<T: DateTime> PeriodEndType<T> for T {}
impl PeriodEndType<Date> for Interval {}
impl PeriodEndType<Timestamp> for Interval {}
impl PeriodEndType<Timestamptz> for Interval {}
impl PeriodEndType<Time> for Interval {}

impl<T, S> Period<T, S> {
    /// `(start, end) OVERLAPS (start, end)`
    pub fn overlaps<S2>(self, other: Period<T, S2>) -> Expr<Bool, Nullable, (S, S2)> {
        Expr::new(Node::binary(
            "OVERLAPS",
            Node::List(self.bounds),
            Node::List(other.bounds),
        ))
    }
}
