//! Typed expressions.

use crate::ast::{Node, Order as OrderNode, Rel};
use crate::ops;
use crate::types::*;
use crate::{Arg, ToPg};
use std::marker::PhantomData;

/// Anything usable where a value of SQL type `Type` is expected: DSL
/// expressions, and Rust values, which become bound parameters.
pub trait Operand {
    type Type: PgType;
    #[doc(hidden)]
    fn into_node(self) -> Node;
}

/// The non-null SQL type of an operand.
pub type Base<E> = <<E as Operand>::Type as PgType>::Base;

/// The nullability of an operand.
pub type Null<E> = <<E as Operand>::Type as PgType>::Null;

/// `O`, nullable when either operand is nullable.
pub type Output<L, R, O> = Strict<O, <L as Operand>::Type, <R as Operand>::Type>;

/// An expression of SQL type `T`.
pub struct Expr<T> {
    node: Node,
    ty: PhantomData<fn() -> T>,
}

impl<T> Expr<T> {
    pub(crate) fn new(node: Node) -> Expr<T> {
        Expr {
            node,
            ty: PhantomData,
        }
    }
}

impl<T> Clone for Expr<T> {
    fn clone(&self) -> Self {
        Expr::new(self.node.clone())
    }
}

impl<T: PgType> Operand for Expr<T> {
    type Type = T;
    fn into_node(self) -> Node {
        self.node
    }
}

impl<T: PgType> Expression for Expr<T> {}

/// A column of a table, view, subquery or function in the FROM clause.
pub struct Column<T> {
    rel: Rel,
    name: &'static str,
    ty: PhantomData<fn() -> T>,
}

impl<T> Clone for Column<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Column<T> {}

impl<T: PgType> Column<T> {
    pub(crate) fn new(rel: Rel, name: &'static str) -> Column<T> {
        Column {
            rel,
            name,
            ty: PhantomData,
        }
    }

    pub(crate) fn rel(&self) -> Rel {
        self.rel
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    /// The same column on the nullable side of an outer join.
    pub fn nullable(self) -> Column<Nullable<T::Base>> {
        Column::new(self.rel, self.name)
    }

    /// `column = value`, for SET clauses.
    pub fn set<V: Operand>(self, value: V) -> Assignment
    where
        V::Type: Assignable<T>,
    {
        Assignment {
            columns: vec![self.name],
            value: value.into_node(),
            rel: self.rel,
        }
    }
}

impl<T: PgType> Operand for Column<T> {
    type Type = T;
    fn into_node(self) -> Node {
        Node::Column(self.rel, self.name)
    }
}

impl<T: PgType> Expression for Column<T> {}

/// A value stored into a column of type `C`: an implicit or assignment cast,
/// and no NULL into a NOT NULL column.
pub trait Assignable<C> {}

impl<V: PgType, C: PgType> Assignable<C> for V
where
    V::Base: Assign<C::Base>,
    V::Null: Fits<C::Null>,
{
}

/// Nullability `Self` is accepted where `N` is expected.
pub trait Fits<N> {}

impl<N: Nullity> Fits<N> for NotNull {}
impl Fits<IsNull> for IsNull {}

/// A `SET` assignment of an UPDATE, MERGE or ON CONFLICT DO UPDATE.
pub struct Assignment {
    pub(crate) columns: Vec<&'static str>,
    pub(crate) value: Node,
    pub(crate) rel: Rel,
}

/// The assignments of a SET clause: one, or a tuple of them.
pub trait Assignments {
    #[doc(hidden)]
    fn into_assignments(self) -> Vec<Assignment>;
}

impl Assignments for Assignment {
    fn into_assignments(self) -> Vec<Assignment> {
        vec![self]
    }
}

/// A declared query parameter, `$N+1`, of type `T`.
pub struct Param<T, const N: usize>(pub(crate) PhantomData<fn() -> T>);

impl<T, const N: usize> Clone for Param<T, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const N: usize> Copy for Param<T, N> {}

impl<T: PgType, const N: usize> Operand for Param<T, N> {
    type Type = T;
    fn into_node(self) -> Node {
        Node::Param(N)
    }
}

impl<T: PgType, const N: usize> Expression for Param<T, N> {}

fn value<T: PgType, V: ToPg<T> + ?Sized>(value: &V) -> Node {
    Node::Value(Arg {
        ty: T::name(),
        value: value.to_pg(),
    })
}

macro_rules! values {
    ($($rust:ty => $pg:ty)*) => {$(
        impl Operand for $rust {
            type Type = $pg;
            fn into_node(self) -> Node {
                value::<$pg, _>(&self)
            }
        }
    )*};
}

values! {
    bool => Bool
    i16 => Int2
    i32 => Int4
    i64 => Int8
    f32 => Float4
    f64 => Float8
    &str => Text
    String => Text
    &String => Text
}

impl<V> Operand for Option<V>
where
    V: Operand + ToPg<V::Type>,
    V::Type: PgType<Null = NotNull>,
{
    type Type = Nullable<V::Type>;
    fn into_node(self) -> Node {
        value::<Nullable<V::Type>, _>(&self)
    }
}

impl<V> Operand for Vec<V>
where
    V: Operand + ToPg<V::Type>,
    V::Type: Element,
{
    type Type = Array<V::Type>;
    fn into_node(self) -> Node {
        value::<Array<V::Type>, _>(&self)
    }
}

impl<V> Operand for &[V]
where
    V: Operand + ToPg<V::Type>,
    V::Type: Element,
{
    type Type = Array<V::Type>;
    fn into_node(self) -> Node {
        value::<Array<V::Type>, _>(self)
    }
}

/// A constant of type `T` in its text input form, as `DATE '2000-01-01'`;
/// bound as a parameter like any value.
pub fn literal<T: PgType<Null = NotNull>>(text: &str) -> Expr<T> {
    Expr::new(Node::Value(Arg {
        ty: T::name(),
        value: Some(text.to_string()),
    }))
}

/// An untyped NULL, accepted wherever a nullable value is.
pub fn null() -> Expr<Nullable<Unknown>> {
    Expr::new(Node::Null)
}

/// An ORDER BY item.
pub struct Order(pub(crate) OrderNode);

impl Order {
    pub fn nulls_first(mut self) -> Order {
        self.0.nulls = Some("FIRST");
        self
    }

    pub fn nulls_last(mut self) -> Order {
        self.0.nulls = Some("LAST");
        self
    }
}

/// A projected expression with an output column name.
pub struct Aliased<E> {
    pub(crate) expr: E,
    pub(crate) name: &'static str,
}

/// A row value, `ROW(a, b)`, of the types in `T`.
pub struct RowValue<T> {
    nodes: Vec<Node>,
    ty: PhantomData<fn() -> T>,
}

impl<T> Operand for RowValue<T> {
    type Type = Record;
    fn into_node(self) -> Node {
        Node::Row(self.nodes)
    }
}

/// Builds `ROW(a, b, ...)` from a tuple of operands.
pub fn row<T: Operands>(values: T) -> RowValue<T::Types> {
    RowValue {
        nodes: values.into_nodes(),
        ty: PhantomData,
    }
}

/// A tuple of operands.
pub trait Operands {
    /// The tuple of SQL types.
    type Types;
    #[doc(hidden)]
    fn into_nodes(self) -> Vec<Node>;
}

/// Element-wise comparability of row values.
pub trait RowCompare<R> {}

impl<T> RowValue<T> {
    pub fn eq<R>(self, other: RowValue<R>) -> Expr<Nullable<Bool>>
    where
        T: RowCompare<R>,
    {
        self.compare("=", other)
    }

    pub fn ne<R>(self, other: RowValue<R>) -> Expr<Nullable<Bool>>
    where
        T: RowCompare<R>,
    {
        self.compare("<>", other)
    }

    pub fn lt<R>(self, other: RowValue<R>) -> Expr<Nullable<Bool>>
    where
        T: RowCompare<R>,
    {
        self.compare("<", other)
    }

    pub fn le<R>(self, other: RowValue<R>) -> Expr<Nullable<Bool>>
    where
        T: RowCompare<R>,
    {
        self.compare("<=", other)
    }

    pub fn gt<R>(self, other: RowValue<R>) -> Expr<Nullable<Bool>>
    where
        T: RowCompare<R>,
    {
        self.compare(">", other)
    }

    pub fn ge<R>(self, other: RowValue<R>) -> Expr<Nullable<Bool>>
    where
        T: RowCompare<R>,
    {
        self.compare(">=", other)
    }

    fn compare<R>(self, op: &'static str, other: RowValue<R>) -> Expr<Nullable<Bool>> {
        Expr::new(Node::binary(op, self.into_node(), other.into_node()))
    }

    /// `ROW(..) IN (subquery)`, with a subquery of matching columns.
    pub fn is_in<Q: crate::Rows>(self, query: Q) -> Expr<Nullable<Bool>>
    where
        T: RowCompare<Q::Row>,
    {
        let list = Node::List(self.nodes);
        Expr::new(Node::binary(
            "IN",
            list,
            Node::Subquery(Box::new(query.into_statement())),
        ))
    }

    /// `(start, end) OVERLAPS (start, end)` for a pair of times.
    pub fn overlaps<R>(self, other: RowValue<R>) -> Expr<Nullable<Bool>>
    where
        T: Period,
        R: Period<Time = T::Time>,
    {
        Expr::new(Node::binary(
            "OVERLAPS",
            Node::List(self.nodes),
            Node::List(other.nodes),
        ))
    }
}

/// A `(start, end)` or `(start, length)` pair for OVERLAPS.
pub trait Period {
    type Time;
}

impl<A: PgType, B: PgType> Period for (A, B)
where
    A::Base: Temporal,
    B::Base: Endpoint<A::Base>,
{
    type Time = A::Base;
}

pub trait Temporal {}
impl Temporal for Date {}
impl Temporal for Time {}
impl Temporal for Timestamp {}
impl Temporal for Timestamptz {}

/// The end of a period starting with a `T`: another `T` or an interval.
pub trait Endpoint<T> {}
impl<T: Temporal> Endpoint<T> for T {}
impl Endpoint<Date> for Interval {}
impl Endpoint<Time> for Interval {}
impl Endpoint<Timestamp> for Interval {}
impl Endpoint<Timestamptz> for Interval {}

/// A list for `IN`: a tuple of operands or a single-column subquery.
pub trait InList<T> {
    /// Whether the list may contain NULL.
    type Null: Nullity;
    #[doc(hidden)]
    fn into_list(self) -> Node;
}

/// Methods available on every DSL expression.
///
/// Methods take `self`: expressions are values consumed into larger ones.
#[allow(clippy::wrong_self_convention)]
pub trait Expression: Operand + Sized {
    crate::ops::methods! {
        binary:
        eq Eq "`=`"
        ne Ne "`<>`"
        lt Lt "`<`"
        le Le "`<=`"
        gt Gt "`>`"
        ge Ge "`>=`"
        above Above "`>^`"
        adjacent Adjacent "`-|-`"
        below Below "`<^`"
        closest_point ClosestPoint "`##`"
        concat Concat "`||`"
        contained_by ContainedBy "`<@`"
        contains Contains "`@>`"
        difference Difference "`-` for ranges"
        distance Distance "`<->`"
        followed_by FollowedBy "`<->` for tsquery"
        get Get "`->`"
        get_path GetPath "`#>`"
        get_path_text GetPathText "`#>>`"
        get_text GetText "`->>`"
        has_all_keys HasAllKeys "`?&`"
        has_any_key HasAnyKey "`?|`"
        has_key HasKey "`?`"
        horizontally_aligned HorizontallyAligned "`?-`"
        ilike Ilike "`ILIKE`"
        image_eq ImageEq "`*=`"
        image_ge ImageGe "`*>=`"
        image_gt ImageGt "`*>`"
        image_le ImageLe "`*<=`"
        image_lt ImageLt "`*<`"
        image_ne ImageNe "`*<>`"
        intersection Intersection "`#` or `*` for ranges"
        intersects Intersects "`?#`"
        iregex Iregex "`~*`"
        is_parallel IsParallel "`?||`"
        is_perpendicular IsPerpendicular "`?-|`"
        left_of LeftOf "`<<`"
        like Like "`LIKE`"
        matches Matches "`@@`"
        matches_alt MatchesAlt "`@@@`"
        not_ilike NotIlike "`NOT ILIKE`"
        not_iregex NotIregex "`!~*`"
        not_like NotLike "`NOT LIKE`"
        not_regex NotRegex "`!~`"
        overabove Overabove "`|&>`"
        overbelow Overbelow "`&<|`"
        overlaps Overlaps "`&&`"
        overleft Overleft "`&<`"
        overright Overright "`&>`"
        pat_ge PatGe "`~>=~`"
        pat_gt PatGt "`~>~`"
        pat_le PatLe "`~<=~`"
        pat_lt PatLt "`~<~`"
        path_exists PathExists "`@?`"
        pow Pow "`^`"
        regex Regex "`~`"
        remove Remove "`-` for jsonb"
        remove_path RemovePath "`#-`"
        right_of RightOf "`>>`"
        same_as SameAs "`~=`"
        starts_with StartsWith "`^@`"
        strictly_above StrictlyAbove "`|>>`"
        strictly_below StrictlyBelow "`<<|`"
        subnet_ge SubnetGe "`>>=`"
        subnet_gt SubnetGt "`>>`"
        subnet_le SubnetLe "`<<=`"
        subnet_lt SubnetLt "`<<`"
        ts_and TsAnd "`&&` for tsquery"
        ts_or TsOr "`||` for tsquery"
        union Union "`+` for ranges"
        vertically_aligned VerticallyAligned "`?|`";
        prefix:
        abs Abs "`@`"
        cbrt Cbrt "`||/`"
        center Center "`@@`"
        is_horizontal IsHorizontal "`?-`"
        is_vertical IsVertical "`?|`"
        length Length "`@-@`"
        negate Negate "`!!`"
        npoints Npoints "`#`"
        sqrt Sqrt "`|/`"
        unary_plus UnaryPlus "`+`";
    }

    fn and<R: Operand>(self, rhs: R) -> Expr<Output<Self, R, Bool>>
    where
        Base<Self>: Boolean,
        Base<R>: Boolean,
    {
        Expr::new(Node::binary("AND", self.into_node(), rhs.into_node()))
    }

    fn or<R: Operand>(self, rhs: R) -> Expr<Output<Self, R, Bool>>
    where
        Base<Self>: Boolean,
        Base<R>: Boolean,
    {
        Expr::new(Node::binary("OR", self.into_node(), rhs.into_node()))
    }

    fn not(self) -> Expr<Self::Type>
    where
        Base<Self>: Boolean,
    {
        Expr::new(Node::Prefix("NOT", Box::new(self.into_node())))
    }

    fn is_null(self) -> Expr<Bool> {
        postfix(self, "IS NULL")
    }

    fn is_not_null(self) -> Expr<Bool> {
        postfix(self, "IS NOT NULL")
    }

    fn is_true(self) -> Expr<Bool>
    where
        Base<Self>: Boolean,
    {
        postfix(self, "IS TRUE")
    }

    fn is_not_true(self) -> Expr<Bool>
    where
        Base<Self>: Boolean,
    {
        postfix(self, "IS NOT TRUE")
    }

    fn is_false(self) -> Expr<Bool>
    where
        Base<Self>: Boolean,
    {
        postfix(self, "IS FALSE")
    }

    fn is_not_false(self) -> Expr<Bool>
    where
        Base<Self>: Boolean,
    {
        postfix(self, "IS NOT FALSE")
    }

    fn is_unknown(self) -> Expr<Bool>
    where
        Base<Self>: Boolean,
    {
        postfix(self, "IS UNKNOWN")
    }

    fn is_not_unknown(self) -> Expr<Bool>
    where
        Base<Self>: Boolean,
    {
        postfix(self, "IS NOT UNKNOWN")
    }

    fn is_distinct_from<R: Operand>(self, rhs: R) -> Expr<Bool>
    where
        Base<Self>: ops::Eq<Base<R>>,
    {
        Expr::new(Node::binary(
            "IS DISTINCT FROM",
            self.into_node(),
            rhs.into_node(),
        ))
    }

    fn is_not_distinct_from<R: Operand>(self, rhs: R) -> Expr<Bool>
    where
        Base<Self>: ops::Eq<Base<R>>,
    {
        Expr::new(Node::binary(
            "IS NOT DISTINCT FROM",
            self.into_node(),
            rhs.into_node(),
        ))
    }

    /// `self BETWEEN low AND high`.
    fn between<L: Operand, H: Operand>(
        self,
        low: L,
        high: H,
    ) -> Expr<Strict<Bool, Output<Self, L, Bool>, H::Type>>
    where
        Base<Self>: ops::Ge<Base<L>> + ops::Le<Base<H>>,
    {
        between(self, "BETWEEN", low, high)
    }

    /// `self BETWEEN SYMMETRIC a AND b`.
    fn between_symmetric<L: Operand, H: Operand>(
        self,
        a: L,
        b: H,
    ) -> Expr<Strict<Bool, Output<Self, L, Bool>, H::Type>>
    where
        Base<Self>: ops::Ge<Base<L>> + ops::Le<Base<H>> + ops::Le<Base<L>> + ops::Ge<Base<H>>,
    {
        between(self, "BETWEEN SYMMETRIC", a, b)
    }

    /// `self IN (list)`, for a tuple of values or a single-column subquery.
    fn is_in<L: InList<Base<Self>>>(
        self,
        list: L,
    ) -> Expr<Either<<Self::Type as PgType>::Null, L::Null, Bool>> {
        Expr::new(Node::binary("IN", self.into_node(), list.into_list()))
    }

    /// `self NOT IN (list)`.
    fn is_not_in<L: InList<Base<Self>>>(
        self,
        list: L,
    ) -> Expr<Either<<Self::Type as PgType>::Null, L::Null, Bool>> {
        Expr::new(Node::binary("NOT IN", self.into_node(), list.into_list()))
    }

    /// `self LIKE pattern ESCAPE escape`.
    fn like_escape<P: Operand, E: Operand>(
        self,
        pattern: P,
        escape: E,
    ) -> Expr<Strict<Bool, Output<Self, P, Bool>, E::Type>>
    where
        Base<Self>: ops::Like<Base<P>>,
        Base<E>: Coerce<Text>,
    {
        Expr::new(Node::Seq(vec![
            self.into_node(),
            Node::Keyword("LIKE"),
            pattern.into_node(),
            Node::Keyword("ESCAPE"),
            escape.into_node(),
        ]))
    }

    /// `self SIMILAR TO pattern`.
    fn similar_to<P: Operand>(self, pattern: P) -> Expr<Output<Self, P, Bool>>
    where
        Base<Self>: Coerce<Text>,
        Base<P>: Coerce<Text>,
    {
        Expr::new(Node::binary(
            "SIMILAR TO",
            self.into_node(),
            pattern.into_node(),
        ))
    }

    /// `CAST(self AS T)`, for casts PostgreSQL defines.
    fn cast<T: PgType<Null = NotNull>>(self) -> Expr<Like<T, Self::Type>>
    where
        Base<Self>: Cast<T>,
    {
        Expr::new(Node::Cast(Box::new(self.into_node()), T::name()))
    }

    /// `CAST(self AS type)`, for a type with modifiers such as `varchar(5)`.
    fn cast_to<T: PgType<Null = NotNull>>(self, ty: SqlType<T>) -> Expr<Like<T, Self::Type>>
    where
        Base<Self>: Cast<T>,
    {
        Expr::new(Node::Cast(Box::new(self.into_node()), ty.name))
    }

    /// `self COLLATE "collation"`.
    fn collate(self, collation: &'static str) -> Expr<Self::Type>
    where
        Base<Self>: Coerce<Text>,
    {
        Expr::new(Node::Collate(Box::new(self.into_node()), collation))
    }

    /// `self AT TIME ZONE zone`.
    fn at_time_zone<Z: Operand>(
        self,
        zone: Z,
    ) -> Expr<Output<Self, Z, <Base<Self> as ZoneShift>::Output>>
    where
        Base<Self>: ZoneShift,
        Base<Z>: Coerce<Text>,
    {
        Expr::new(Node::binary(
            "AT TIME ZONE",
            self.into_node(),
            zone.into_node(),
        ))
    }

    /// `self[index]`, an array element; NULL when out of bounds.
    fn at<I: Operand>(self, index: I) -> Expr<Nullable<<Base<Self> as Indexed>::Element>>
    where
        Base<Self>: Indexed,
        Base<I>: Coerce<Int4>,
    {
        Expr::new(Node::Subscript(
            Box::new(self.into_node()),
            Box::new(index.into_node()),
            None,
        ))
    }

    /// `self[low:high]`, an array slice.
    fn slice<L: Operand, H: Operand>(self, low: L, high: H) -> Expr<Self::Type>
    where
        Base<Self>: Indexed,
        Base<L>: Coerce<Int4>,
        Base<H>: Coerce<Int4>,
    {
        Expr::new(Node::Subscript(
            Box::new(self.into_node()),
            Box::new(low.into_node()),
            Some(Box::new(high.into_node())),
        ))
    }

    fn asc(self) -> Order {
        order(self, Some("ASC"))
    }

    fn desc(self) -> Order {
        order(self, Some("DESC"))
    }

    fn nulls_first(self) -> Order {
        order(self, None).nulls_first()
    }

    fn nulls_last(self) -> Order {
        order(self, None).nulls_last()
    }

    /// Names the output column of this projected expression.
    fn alias(self, name: &'static str) -> Aliased<Self> {
        Aliased { expr: self, name }
    }
}

/// Types with `AT TIME ZONE`, which converts between local and zoned times.
pub trait ZoneShift {
    type Output: PgType<Null = NotNull>;
}

impl ZoneShift for Timestamp {
    type Output = Timestamptz;
}

impl ZoneShift for Timestamptz {
    type Output = Timestamp;
}

/// Arrays, which can be subscripted.
pub trait Indexed {
    type Element: PgType<Null = NotNull>;
}

impl<T: Element> Indexed for Array<T> {
    type Element = T;
}

macro_rules! std_ops {
    ($($g:tt $ty:ty;)*) => {$(
        std_ops!(@binary $g $ty:
            Add add Plus, Sub sub Minus, Mul mul Mul, Div div Div, Rem rem Rem,
            BitAnd bitand BitAnd, BitOr bitor BitOr, BitXor bitxor BitXor, Shl shl Shl, Shr shr Shr);
        std_ops!(@prefix $g $ty: Neg neg Neg, Not not BitNot);
    )*};
    (@binary $g:tt $ty:ty: $($std:ident $method:ident $op:ident),*) => {$(
        std_ops!(@one_binary $g $ty: $std $method $op);
    )*};
    (@one_binary [$($g:tt)*] $ty:ty: $std:ident $method:ident $op:ident) => {
        impl<$($g)*, R: Operand> std::ops::$std<R> for $ty
        where
            Base<Self>: ops::$op<Base<R>>,
        {
            type Output = Expr<Output<Self, R, <Base<Self> as ops::$op<Base<R>>>::Output>>;
            fn $method(self, rhs: R) -> Self::Output {
                let op = <Base<Self> as ops::$op<Base<R>>>::OP;
                Expr::new(Node::binary(op, self.into_node(), rhs.into_node()))
            }
        }
    };
    (@prefix $g:tt $ty:ty: $($std:ident $method:ident $op:ident),*) => {$(
        std_ops!(@one_prefix $g $ty: $std $method $op);
    )*};
    (@one_prefix [$($g:tt)*] $ty:ty: $std:ident $method:ident $op:ident) => {
        impl<$($g)*> std::ops::$std for $ty
        where
            Base<Self>: ops::$op,
        {
            type Output = Expr<Like<<Base<Self> as ops::$op>::Output, <Self as Operand>::Type>>;
            fn $method(self) -> Self::Output {
                let op = <Base<Self> as ops::$op>::OP;
                Expr::new(Node::Prefix(op, Box::new(self.into_node())))
            }
        }
    };
}

std_ops! {
    [T: PgType] Column<T>;
    [T: PgType] Expr<T>;
    [T: PgType, const N: usize] Param<T, N>;
}

/// A type with modifiers, for `cast_to`.
pub struct SqlType<T> {
    name: String,
    ty: PhantomData<T>,
}

fn sql_type<T: PgType>(modifiers: &[u32]) -> SqlType<T> {
    let modifiers: Vec<String> = modifiers.iter().map(u32::to_string).collect();
    SqlType {
        name: format!("{}({})", T::name(), modifiers.join(", ")),
        ty: PhantomData,
    }
}

/// `varchar(length)`.
pub fn varchar(length: u32) -> SqlType<Varchar> {
    sql_type(&[length])
}

/// `numeric(precision, scale)`.
pub fn numeric(precision: u32, scale: u32) -> SqlType<Numeric> {
    sql_type(&[precision, scale])
}

/// A tuple of columns assigned together: `(a, b).set(row((x, y)))` or
/// `(a, b).set(subquery)`.
pub trait ColumnList: Sized {
    /// The tuple of column types.
    type Types;
    #[doc(hidden)]
    fn names(&self) -> (Vec<&'static str>, Rel);

    fn set<V: RowSource<Self::Types>>(self, value: V) -> Assignment {
        let (columns, rel) = self.names();
        Assignment {
            columns,
            value: value.into_row_node(),
            rel,
        }
    }
}

/// A row assignable to columns of the types `C`.
pub trait RowSource<C> {
    #[doc(hidden)]
    fn into_row_node(self) -> Node;
}

/// Row types whose values can be stored in columns of the types `C`.
pub trait AssignRow<C> {}

impl<C, Q: crate::Rows> RowSource<C> for Q
where
    Q::Row: AssignRow<C>,
{
    fn into_row_node(self) -> Node {
        Node::Subquery(Box::new(self.into_statement()))
    }
}

impl<C, T: AssignRow<C>> RowSource<C> for RowValue<T> {
    fn into_row_node(self) -> Node {
        self.into_node()
    }
}

fn postfix<E: Operand, T>(operand: E, op: &'static str) -> Expr<T> {
    Expr::new(Node::Postfix(Box::new(operand.into_node()), op))
}

fn between<E: Operand, L: Operand, H: Operand, T>(
    operand: E,
    op: &'static str,
    low: L,
    high: H,
) -> Expr<T> {
    Expr::new(Node::Seq(vec![
        operand.into_node(),
        Node::Keyword(op),
        low.into_node(),
        Node::Keyword("AND"),
        high.into_node(),
    ]))
}

fn order<E: Operand>(operand: E, direction: Option<&'static str>) -> Order {
    Order(OrderNode {
        node: operand.into_node(),
        direction,
        nulls: None,
    })
}
