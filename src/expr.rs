//! Typed expressions: every expression carries its SQL type, nullability and
//! the sources it references (its scope) in its Rust type, and an untyped
//! [`Node`] describing its structure.

use crate::op::{self, Symbol};
use crate::query::Stmt;
use crate::types::*;
use std::marker::PhantomData;
use std::rc::Rc;

/// Untyped expression syntax tree.
#[derive(Clone, Debug)]
pub enum Node {
    /// `qualifier.name`, or just `name` when the qualifier is empty.
    Column(&'static str, &'static str),
    Param(Rc<Param>),
    /// `'text'::type`, or an untyped string constant when the type is empty.
    Literal(String, String),
    /// A type name.
    Type(String),
    /// A keyword or other fixed token, written as is.
    Keyword(&'static str),
    /// A quoted identifier.
    Ident(&'static str),
    /// `(left op right)`
    Binary(&'static str, Box<[Node; 2]>),
    /// `(op operand)`
    Prefix(&'static str, Box<Node>),
    /// `(operand op)`
    Postfix(Box<Node>, &'static str),
    Call(Box<Call>),
    /// `CAST(node AS type)`
    Cast(Box<Node>, String),
    /// `(node).field`
    Field(Box<Node>, &'static str),
    /// `(node)[a]` or `(node)[a:b]`
    Index(Box<Node>, Vec<Node>),
    /// `ARRAY[...]`
    Array(Vec<Node>),
    /// `(a, b, ...)`
    List(Vec<Node>),
    /// Space separated sequence, for special syntax such as `CASE`.
    Seq(Vec<Node>),
    /// `(SELECT ...)`
    Subquery(Box<Stmt>),
}

/// A function call with optional aggregate and window clauses.
#[derive(Clone, Debug, Default)]
pub struct Call {
    pub name: &'static str,
    pub args: Vec<Node>,
    pub distinct: bool,
    pub order: Vec<Node>,
    pub within_group: Vec<Node>,
    pub filter: Option<Node>,
    pub over: Option<Window>,
}

/// A window specification: `OVER name` or `OVER (name PARTITION BY ... ORDER BY ... frame)`.
#[derive(Clone, Debug, Default)]
pub struct Window {
    pub base: Option<&'static str>,
    pub partition: Vec<Node>,
    pub order: Vec<Node>,
    pub frame: Vec<Node>,
}

/// A bound parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub sql_type: String,
    pub value: Value,
}

/// A parameter value in PostgreSQL text format.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Text(String),
    /// Supplied by the caller at execution time.
    Placeholder,
}

impl Node {
    pub(crate) fn call(name: &'static str, args: Vec<Node>) -> Node {
        Node::Call(Box::new(Call {
            name,
            args,
            ..Call::default()
        }))
    }

    pub(crate) fn binary(op: &'static str, left: Node, right: Node) -> Node {
        Node::Binary(op, Box::new([left, right]))
    }

    pub(crate) fn param<V: Bind>(value: &V) -> Node {
        let value_text = value.encode().map_or(Value::Null, Value::Text);
        Node::Param(Rc::new(Param {
            sql_type: value.param_type(),
            value: value_text,
        }))
    }
}

macro_rules! operator_methods {
    ($($method:ident => $op:ident),* $(,)?) => {$(
        #[doc = concat!("`self ", stringify!($op), " right`, see [`op::", stringify!($op), "`].")]
        fn $method<R: Operand>(self, right: R) -> Binary<Self, op::$op, R>
        where
            Self::Sql: Operator<op::$op, R::Sql>,
        {
            binary(self, right)
        }
    )*};
}

macro_rules! prefix_methods {
    ($($method:ident => $op:ident),* $(,)?) => {$(
        #[doc = concat!("Prefix operator, see [`op::", stringify!($op), "`].")]
        fn $method(self) -> Unary<Self, op::$op>
        where
            Self::Sql: Prefix<op::$op>,
        {
            unary(self)
        }
    )*};
}

/// A typed SQL expression.
pub trait Expression: Sized {
    /// The SQL type of the expression.
    type Sql: SqlType;
    /// Whether the expression may be NULL.
    type Null: Nullability;
    /// The sources the expression references.
    type Scope;

    fn into_node(self) -> Node;
}

/// Operators and predicates on SQL expressions. Implemented by expression
/// types, not by Rust values, whose own methods it would otherwise shadow.
pub trait ExpressionMethods: Expression {
    fn eq<R: Operand>(self, right: R) -> Binary<Self, op::Eq, R>
    where
        Self::Sql: Operator<op::Eq, R::Sql>,
    {
        binary(self, right)
    }

    fn ne<R: Operand>(self, right: R) -> Binary<Self, op::Ne, R>
    where
        Self::Sql: Operator<op::Ne, R::Sql>,
    {
        binary(self, right)
    }

    fn lt<R: Operand>(self, right: R) -> Binary<Self, op::Lt, R>
    where
        Self::Sql: Operator<op::Lt, R::Sql>,
    {
        binary(self, right)
    }

    fn le<R: Operand>(self, right: R) -> Binary<Self, op::Le, R>
    where
        Self::Sql: Operator<op::Le, R::Sql>,
    {
        binary(self, right)
    }

    fn gt<R: Operand>(self, right: R) -> Binary<Self, op::Gt, R>
    where
        Self::Sql: Operator<op::Gt, R::Sql>,
    {
        binary(self, right)
    }

    fn ge<R: Operand>(self, right: R) -> Binary<Self, op::Ge, R>
    where
        Self::Sql: Operator<op::Ge, R::Sql>,
    {
        binary(self, right)
    }

    /// `self AND right`
    fn and<R: Predicate>(self, right: R) -> Logical<Self, R>
    where
        Self: Predicate,
    {
        Expr::new(Node::binary("AND", self.into_node(), right.into_node()))
    }

    /// `self OR right`
    fn or<R: Predicate>(self, right: R) -> Logical<Self, R>
    where
        Self: Predicate,
    {
        Expr::new(Node::binary("OR", self.into_node(), right.into_node()))
    }

    fn is_null(self) -> Expr<Bool, NotNull, Self::Scope> {
        postfix(self, "IS NULL")
    }

    fn is_not_null(self) -> Expr<Bool, NotNull, Self::Scope> {
        postfix(self, "IS NOT NULL")
    }

    fn is_true(self) -> Expr<Bool, NotNull, Self::Scope>
    where
        Self: Predicate,
    {
        postfix(self, "IS TRUE")
    }

    fn is_not_true(self) -> Expr<Bool, NotNull, Self::Scope>
    where
        Self: Predicate,
    {
        postfix(self, "IS NOT TRUE")
    }

    fn is_false(self) -> Expr<Bool, NotNull, Self::Scope>
    where
        Self: Predicate,
    {
        postfix(self, "IS FALSE")
    }

    fn is_not_false(self) -> Expr<Bool, NotNull, Self::Scope>
    where
        Self: Predicate,
    {
        postfix(self, "IS NOT FALSE")
    }

    fn is_unknown(self) -> Expr<Bool, NotNull, Self::Scope>
    where
        Self: Predicate,
    {
        postfix(self, "IS UNKNOWN")
    }

    fn is_distinct_from<R: Expression>(
        self,
        right: R,
    ) -> Expr<Bool, NotNull, (Self::Scope, R::Scope)>
    where
        Self::Sql: Operator<op::Eq, R::Sql>,
    {
        Expr::new(Node::binary(
            "IS DISTINCT FROM",
            self.into_node(),
            right.into_node(),
        ))
    }

    fn is_not_distinct_from<R: Expression>(
        self,
        right: R,
    ) -> Expr<Bool, NotNull, (Self::Scope, R::Scope)>
    where
        Self::Sql: Operator<op::Eq, R::Sql>,
    {
        Expr::new(Node::binary(
            "IS NOT DISTINCT FROM",
            self.into_node(),
            right.into_node(),
        ))
    }

    /// `self BETWEEN low AND high`
    fn between<A: Arg<Self::Sql>, B: Arg<Self::Sql>>(self, low: A, high: B) -> Between<Self, A, B>
    where
        Self::Sql: Operator<op::Le, Self::Sql>,
    {
        between(self, "BETWEEN", low, high)
    }

    /// `self BETWEEN SYMMETRIC a AND b`
    fn between_symmetric<A: Arg<Self::Sql>, B: Arg<Self::Sql>>(
        self,
        a: A,
        b: B,
    ) -> Between<Self, A, B>
    where
        Self::Sql: Operator<op::Le, Self::Sql>,
    {
        between(self, "BETWEEN SYMMETRIC", a, b)
    }

    /// `self IN (...)`, for a list of values or a subquery.
    fn in_<L: InList<Self::Sql>>(self, list: L) -> InExpr<Self, L> {
        Expr::new(Node::binary("IN", self.into_node(), list.into_node()))
    }

    /// `self NOT IN (...)`, for a list of values or a subquery.
    fn not_in<L: InList<Self::Sql>>(self, list: L) -> InExpr<Self, L> {
        Expr::new(Node::binary("NOT IN", self.into_node(), list.into_node()))
    }

    /// `self LIKE pattern ESCAPE escape`
    fn like_escape<P: Arg<Text>, E: Arg<Text>>(
        self,
        pattern: P,
        escape: E,
    ) -> LikeEscape<Self, P, E>
    where
        Self::Sql: Operator<op::Like, Text>,
    {
        let pattern = Node::Seq(vec![
            pattern.into_arg(),
            Node::Keyword("ESCAPE"),
            escape.into_arg(),
        ]);
        Expr::new(Node::binary("LIKE", self.into_node(), pattern))
    }

    /// `self SIMILAR TO pattern`
    fn similar_to<P: Arg<Text>>(
        self,
        pattern: P,
    ) -> Expr<Bool, <Self::Null as Nullability>::Or<P::Null>, (Self::Scope, P::Scope)>
    where
        Self::Sql: Coerce<Text>,
    {
        Expr::new(Node::binary(
            "SIMILAR TO",
            self.into_node(),
            pattern.into_arg(),
        ))
    }

    /// `CAST(self AS T)`
    fn cast<T: CastTarget>(self) -> Expr<T::Sql, Self::Null, Self::Scope>
    where
        Self::Sql: CastTo<T::Sql>,
    {
        let mut name = String::new();
        T::write_name(&mut name);
        Expr::new(Node::Cast(Box::new(self.into_node()), name))
    }

    /// `self COLLATE "collation"`
    fn collate(self, collation: &'static str) -> Expr<Self::Sql, Self::Null, Self::Scope>
    where
        Self::Sql: Coerce<Text>,
    {
        Expr::new(Node::binary(
            "COLLATE",
            self.into_node(),
            Node::Ident(collation),
        ))
    }

    /// `self AT TIME ZONE zone`
    fn at_time_zone<Z: Arg<Text>>(
        self,
        zone: Z,
    ) -> Expr<
        <Self::Sql as TimeZone>::Out,
        <Self::Null as Nullability>::Or<Z::Null>,
        (Self::Scope, Z::Scope),
    >
    where
        Self::Sql: TimeZone,
    {
        Expr::new(Node::binary(
            "AT TIME ZONE",
            self.into_node(),
            zone.into_arg(),
        ))
    }

    /// Array element `(self)[index]`; NULL when out of bounds.
    fn at<I: Arg<Int4>>(
        self,
        index: I,
    ) -> Expr<<Self::Sql as Element>::Item, Nullable, (Self::Scope, I::Scope)>
    where
        Self::Sql: Element,
    {
        Expr::new(Node::Index(
            Box::new(self.into_node()),
            vec![index.into_arg()],
        ))
    }

    /// Array slice `(self)[low:high]`.
    fn slice<A: Arg<Int4>, B: Arg<Int4>>(
        self,
        low: A,
        high: B,
    ) -> Expr<Self::Sql, Nullable, (Self::Scope, (A::Scope, B::Scope))>
    where
        Self::Sql: Element,
    {
        let bounds = vec![low.into_arg(), Node::Keyword(":"), high.into_arg()];
        Expr::new(Node::Index(Box::new(self.into_node()), bounds))
    }

    /// `self IS JSON`
    fn is_json(self) -> Expr<Bool, Self::Null, Self::Scope>
    where
        Self::Sql: JsonInput,
    {
        postfix(self, "IS JSON")
    }

    /// `self IS NOT JSON`
    fn is_not_json(self) -> Expr<Bool, Self::Null, Self::Scope>
    where
        Self::Sql: JsonInput,
    {
        postfix(self, "IS NOT JSON")
    }

    /// `self IS JSON OBJECT`
    fn is_json_object(self) -> Expr<Bool, Self::Null, Self::Scope>
    where
        Self::Sql: JsonInput,
    {
        postfix(self, "IS JSON OBJECT")
    }

    /// `self IS JSON ARRAY`
    fn is_json_array(self) -> Expr<Bool, Self::Null, Self::Scope>
    where
        Self::Sql: JsonInput,
    {
        postfix(self, "IS JSON ARRAY")
    }

    /// `self IS JSON SCALAR`
    fn is_json_scalar(self) -> Expr<Bool, Self::Null, Self::Scope>
    where
        Self::Sql: JsonInput,
    {
        postfix(self, "IS JSON SCALAR")
    }

    /// `self IS JSON WITH UNIQUE KEYS`
    fn is_json_with_unique_keys(self) -> Expr<Bool, Self::Null, Self::Scope>
    where
        Self::Sql: JsonInput,
    {
        postfix(self, "IS JSON WITH UNIQUE KEYS")
    }

    /// Names the expression in a projection: `self AS name`.
    fn as_(self, name: &'static str) -> crate::query::As<Self> {
        crate::query::As(self, name)
    }

    /// Ascending sort key.
    fn asc(self) -> Ordered<Self::Scope> {
        Ordered::new(self.into_node(), "ASC")
    }

    /// Descending sort key.
    fn desc(self) -> Ordered<Self::Scope> {
        Ordered::new(self.into_node(), "DESC")
    }

    /// Sort key with NULLs first.
    fn nulls_first(self) -> Ordered<Self::Scope> {
        Ordered::new(self.into_node(), "NULLS FIRST")
    }

    /// Sort key with NULLs last.
    fn nulls_last(self) -> Ordered<Self::Scope> {
        Ordered::new(self.into_node(), "NULLS LAST")
    }

    /// Sort key ordered by an operator: `self USING >`.
    fn using<Op: Symbol>(self, _op: Op) -> Ordered<Self::Scope>
    where
        Self::Sql: Operator<Op, Self::Sql, Out = Bool>,
    {
        let node = self.into_node();
        Ordered {
            node: Node::Seq(vec![node, Node::Keyword("USING"), Node::Keyword(Op::SQL)]),
            _scope: PhantomData,
        }
    }

    operator_methods! {
        pow => Pow, concat => Concat, like => Like, ilike => ILike, not_like => NotLike,
        not_ilike => NotILike, regex => Regex, iregex => IRegex, not_regex => NotRegex,
        not_iregex => NotIRegex, contains => Contains, contained_by => ContainedBy,
        contains_or_equals => ContainsOrEquals, contained_by_or_equals => ContainedByOrEquals,
        overlaps => Overlaps, overleft => Overleft, overright => Overright,
        overbelow => Overbelow, overabove => Overabove, strictly_below => StrictlyBelow,
        strictly_above => StrictlyAbove, below => Below, above => Above, adjacent => Adjacent,
        distance => Distance, intersects => Intersects, horizontal => Horizontal,
        perpendicular => Perpendicular, parallel => Parallel, vertical => HasAnyKey,
        closest_point => ClosestPoint, same_as => SameAs, matches => Matches,
        matches_deprecated => MatchesDeprecated, starts_with => StartsWith, get => Get,
        get_text => GetText, get_path => GetPath, get_path_text => GetPathText,
        delete_path => DeletePath, has_key => HasKey, has_any_key => HasAnyKey,
        has_all_keys => HasAllKeys, path_exists => PathExists, rec_eq => RecEq,
        rec_ne => RecNe, rec_lt => RecLt, rec_le => RecLe, rec_gt => RecGt, rec_ge => RecGe,
        pattern_lt => PatternLt, pattern_le => PatternLe, pattern_gt => PatternGt,
        pattern_ge => PatternGe,
    }

    prefix_methods! {
        unary_plus => UnaryPlus, abs => Abs, sqrt => Sqrt, cbrt => Cbrt, length => Length,
        center => Center, npoints => Npoints, is_horizontal => IsHorizontal,
        is_vertical => IsVertical,
    }
}

/// The right operand of an operator: an expression or `ANY`/`ALL`.
pub trait Operand {
    type Sql: SqlType;
    type Null: Nullability;
    type Scope;
    fn into_operand(self) -> Node;
}

impl<E: Expression> Operand for E {
    type Sql = E::Sql;
    type Null = E::Null;
    type Scope = E::Scope;
    fn into_operand(self) -> Node {
        Expression::into_node(self)
    }
}

/// An argument accepted where SQL type `T` is expected: an expression whose
/// type implicitly coerces to `T`, `NULL`, or a Rust value with a natural
/// encoding as `T`.
pub trait Arg<T> {
    type Null: Nullability;
    type Scope;
    fn into_arg(self) -> Node;
}

impl<E: Expression, T: SqlType> Arg<T> for E
where
    E::Sql: Coerce<T>,
{
    type Null = E::Null;
    type Scope = E::Scope;
    fn into_arg(self) -> Node {
        self.into_node()
    }
}

macro_rules! text_args {
    ($($t:ty),*) => {$(
        /// A string in the text format of the type.
        impl Arg<$t> for &str {
            type Null = NotNull;
            type Scope = ();
            fn into_arg(self) -> Node {
                Node::Param(Rc::new(Param { sql_type: type_name::<$t>(), value: Value::Text(self.to_string()) }))
            }
        }
    )*};
}

text_args!(Json, Jsonb, Jsonpath, Xml, Tsquery, Tsvector);

/// `NULL`, typed by the context it is used in.
pub struct Null;

/// `NULL`, typed by the context it is used in.
pub fn null() -> Null {
    Null
}

impl<T: SqlType> Arg<T> for Null {
    type Null = Nullable;
    type Scope = ();
    fn into_arg(self) -> Node {
        Node::Cast(Box::new(Node::Keyword("NULL")), type_name::<T>())
    }
}

/// A parameter whose value is supplied when the statement is executed; its
/// type is the type expected by the context.
pub struct Placeholder;

/// A parameter whose value is supplied when the statement is executed.
pub fn placeholder() -> Placeholder {
    Placeholder
}

impl<T: SqlType> Arg<T> for Placeholder {
    type Null = Nullable;
    type Scope = ();
    fn into_arg(self) -> Node {
        Node::Param(Rc::new(Param {
            sql_type: type_name::<T>(),
            value: Value::Placeholder,
        }))
    }
}

/// Implicit coercion of `Self` to `T`, as PostgreSQL applies to function
/// arguments and assignments.
pub trait Coerce<T> {}
impl<T: SqlType> Coerce<T> for T {}

/// Boolean SQL types.
pub trait Boolean {}
impl Boolean for Bool {}

/// A boolean expression, as required by `WHERE`, `HAVING`, `ON` and `FILTER`.
pub trait Predicate: Expression {}
impl<E: Expression> Predicate for E where E::Sql: Boolean {}

/// `Self` and `T` are the same SQL type.
pub trait SameType<T> {}
impl<T: SqlType> SameType<T> for T {}

/// An uninhabited operand type. Its blanket impls give every type a second
/// candidate impl of [`Operator`], [`SameType`] and
/// [`Unify`](crate::Unify), so that rustc reports a mismatch as an unmet
/// bound such as `Jsonb: Operator<Contains, Int4>` (E0277) rather than
/// inferring the operand type from the only impl.
pub enum Unreachable {}
impl<T> SameType<Unreachable> for T {}
impl<L, Op> Operator<Op, Unreachable> for L {
    type Out = Bool;
}

/// Explicit cast of `Self` to `T`.
pub trait CastTo<T> {}
impl<T: SqlType> CastTo<T> for T {}

/// Binary operator `Op` with right operand type `R`.
pub trait Operator<Op, R> {
    type Out: SqlType;
}

/// Prefix operator `Op`.
pub trait Prefix<Op> {
    type Out: SqlType;
    const SQL: &'static str;
}

impl Prefix<op::Not> for Bool {
    type Out = Bool;
    const SQL: &'static str = "NOT";
}

/// Types accepted by `IS JSON` and SQL/JSON functions.
pub trait JsonInput {}
impl JsonInput for Text {}
impl JsonInput for Varchar {}
impl JsonInput for Json {}
impl JsonInput for Jsonb {}
impl JsonInput for Bytea {}

/// Types supporting `AT TIME ZONE`.
pub trait TimeZone {
    type Out: SqlType;
}
impl TimeZone for Timestamp {
    type Out = Timestamptz;
}
impl TimeZone for Timestamptz {
    type Out = Timestamp;
}
impl TimeZone for Timetz {
    type Out = Timetz;
}

/// Types with subscripts.
pub trait Element {
    type Item: SqlType;
}
impl<T: SqlType> Element for Array<T> {
    type Item = T;
}

/// The result of binary operator `Op`.
pub type Binary<L, Op, R> = Expr<
    <<L as Expression>::Sql as Operator<Op, <R as Operand>::Sql>>::Out,
    <<L as Expression>::Null as Nullability>::Or<<R as Operand>::Null>,
    (<L as Expression>::Scope, <R as Operand>::Scope),
>;

/// The result of prefix operator `Op`.
pub type Unary<E, Op> = Expr<
    <<E as Expression>::Sql as Prefix<Op>>::Out,
    <E as Expression>::Null,
    <E as Expression>::Scope,
>;

/// The result of `AND`/`OR`.
pub type Logical<L, R> = Expr<
    Bool,
    <<L as Expression>::Null as Nullability>::Or<<R as Expression>::Null>,
    (<L as Expression>::Scope, <R as Expression>::Scope),
>;

/// The result of `BETWEEN`.
pub type Between<E, A, B> = Expr<
    Bool,
    <<<E as Expression>::Null as Nullability>::Or<<A as Arg<<E as Expression>::Sql>>::Null> as Nullability>::Or<
        <B as Arg<<E as Expression>::Sql>>::Null,
    >,
    (<E as Expression>::Scope, (<A as Arg<<E as Expression>::Sql>>::Scope, <B as Arg<<E as Expression>::Sql>>::Scope)),
>;

/// The result of `IN`.
pub type InExpr<E, L> = Expr<
    Bool,
    <<E as Expression>::Null as Nullability>::Or<<L as InList<<E as Expression>::Sql>>::Null>,
    (
        <E as Expression>::Scope,
        <L as InList<<E as Expression>::Sql>>::Scope,
    ),
>;

/// The result of `LIKE ... ESCAPE`.
pub type LikeEscape<E, P, X> = Expr<
    Bool,
    <<<E as Expression>::Null as Nullability>::Or<<P as Arg<Text>>::Null> as Nullability>::Or<
        <X as Arg<Text>>::Null,
    >,
    (
        <E as Expression>::Scope,
        (<P as Arg<Text>>::Scope, <X as Arg<Text>>::Scope),
    ),
>;

pub(crate) fn binary<L: Expression, Op: Symbol, R: Operand>(left: L, right: R) -> Binary<L, Op, R>
where
    L::Sql: Operator<Op, R::Sql>,
{
    Expr::new(Node::binary(
        Op::SQL,
        left.into_node(),
        right.into_operand(),
    ))
}

pub(crate) fn unary<E: Expression, Op>(operand: E) -> Unary<E, Op>
where
    E::Sql: Prefix<Op>,
{
    Expr::new(Node::Prefix(
        <E::Sql as Prefix<Op>>::SQL,
        Box::new(operand.into_node()),
    ))
}

fn postfix<E: Expression, T, N>(operand: E, op: &'static str) -> Expr<T, N, E::Scope> {
    Expr::new(Node::Postfix(Box::new(operand.into_node()), op))
}

fn between<E: Expression, A: Arg<E::Sql>, B: Arg<E::Sql>>(
    e: E,
    op: &'static str,
    a: A,
    b: B,
) -> Between<E, A, B> {
    let bounds = Node::Seq(vec![a.into_arg(), Node::Keyword("AND"), b.into_arg()]);
    Expr::new(Node::binary(op, e.into_node(), bounds))
}

/// Right-hand side of `IN`: a list of values or a single-column subquery.
pub trait InList<T> {
    type Null: Nullability;
    type Scope;
    fn into_node(self) -> Node;
}

impl<T, A: Arg<T>, const N: usize> InList<T> for [A; N] {
    type Null = A::Null;
    type Scope = A::Scope;
    fn into_node(self) -> Node {
        Node::List(self.into_iter().map(Arg::into_arg).collect())
    }
}

impl<T, A: Arg<T> + Clone> InList<T> for &[A] {
    type Null = A::Null;
    type Scope = A::Scope;
    fn into_node(self) -> Node {
        Node::List(self.iter().cloned().map(Arg::into_arg).collect())
    }
}

impl<T, A: Arg<T>> InList<T> for Vec<A> {
    type Null = A::Null;
    type Scope = A::Scope;
    fn into_node(self) -> Node {
        Node::List(self.into_iter().map(Arg::into_arg).collect())
    }
}

/// A typed expression.
pub struct Expr<T, N = NotNull, S = ()> {
    node: Node,
    _type: PhantomData<fn() -> (T, N, S)>,
}

impl<T, N, S> Expr<T, N, S> {
    pub(crate) fn new(node: Node) -> Self {
        Expr {
            node,
            _type: PhantomData,
        }
    }
}

impl<T, N, S> Clone for Expr<T, N, S> {
    fn clone(&self) -> Self {
        Expr::new(self.node.clone())
    }
}

impl<T: SqlType, N: Nullability, S> Expression for Expr<T, N, S> {
    type Sql = T;
    type Null = N;
    type Scope = S;
    fn into_node(self) -> Node {
        self.node
    }
}

impl<V: Bind> Expression for V {
    type Sql = V::Sql;
    type Null = V::Null;
    type Scope = ();
    fn into_node(self) -> Node {
        Node::param(&self)
    }
}

/// A Rust value as a SQL parameter; clones share one parameter slot.
pub fn param<V: Bind>(value: V) -> Expr<V::Sql, V::Null> {
    Expr::new(Node::param(&value))
}

/// A Rust value as a SQL literal, for clauses that only accept constants.
pub fn literal<V: Bind>(value: V) -> Expr<V::Sql, V::Null> {
    match value.encode() {
        Some(text) => Expr::new(Node::Literal(text, value.param_type())),
        None => Expr::new(Node::Cast(
            Box::new(Node::Keyword("NULL")),
            value.param_type(),
        )),
    }
}

/// Identifies the source a column belongs to and whether that source may be
/// NULL-extended by an outer join.
pub trait Qualifier: 'static {
    type Null: Nullability;
}

/// A column of source `Q`, of SQL type `T` and declared nullability `N`.
pub struct Column<T, N, Q> {
    pub(crate) qualifier: &'static str,
    pub(crate) name: &'static str,
    _type: PhantomData<fn() -> (T, N, Q)>,
}

impl<T, N, Q> Column<T, N, Q> {
    pub const fn new(qualifier: &'static str, name: &'static str) -> Self {
        Column {
            qualifier,
            name,
            _type: PhantomData,
        }
    }
}

impl<T, N, Q> Clone for Column<T, N, Q> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, N, Q> Copy for Column<T, N, Q> {}

impl<T: SqlType, N: Nullability, Q: Qualifier> Expression for Column<T, N, Q> {
    type Sql = T;
    type Null = <Q::Null as Nullability>::Or<N>;
    type Scope = In<Q>;
    fn into_node(self) -> Node {
        Node::Column(self.qualifier, self.name)
    }
}

/// Scope leaf: a reference to source `Q`.
pub struct In<Q>(PhantomData<Q>);

/// A sort key.
pub struct Ordered<S> {
    pub(crate) node: Node,
    _scope: PhantomData<fn() -> S>,
}

impl<S> Ordered<S> {
    fn new(node: Node, modifier: &'static str) -> Self {
        Ordered {
            node: Node::Seq(vec![node, Node::Keyword(modifier)]),
            _scope: PhantomData,
        }
    }

    fn push(mut self, modifier: &'static str) -> Self {
        if let Node::Seq(parts) = &mut self.node {
            parts.push(Node::Keyword(modifier));
        }
        self
    }

    pub fn nulls_first(self) -> Self {
        self.push("NULLS FIRST")
    }

    pub fn nulls_last(self) -> Self {
        self.push("NULLS LAST")
    }
}

/// Searched `CASE`: starts with its first `WHEN condition THEN value`.
pub fn when<C: Arg<Bool>, V: Expression>(
    condition: C,
    value: V,
) -> Case<Bool, V::Sql, V::Null, (C::Scope, V::Scope)> {
    let parts = vec![
        Node::Keyword("CASE"),
        Node::Keyword("WHEN"),
        condition.into_arg(),
        Node::Keyword("THEN"),
        value.into_node(),
    ];
    Case {
        parts,
        _type: PhantomData,
    }
}

/// Simple `CASE operand WHEN value THEN result ...`.
pub fn case<E: Expression>(operand: E) -> CaseOf<E::Sql, E::Scope> {
    CaseOf(Case {
        parts: vec![Node::Keyword("CASE"), operand.into_node()],
        _type: PhantomData,
    })
}

/// `CASE operand` before its first `WHEN`.
pub struct CaseOf<O, S>(Case<O, Bool, NotNull, S>);

impl<O, S> CaseOf<O, S> {
    pub fn when<M: Arg<O>, V: Expression>(
        self,
        matching: M,
        value: V,
    ) -> Case<O, V::Sql, V::Null, (S, (M::Scope, V::Scope))> {
        let Case { mut parts, .. } = self.0;
        parts.extend([
            Node::Keyword("WHEN"),
            matching.into_arg(),
            Node::Keyword("THEN"),
            value.into_node(),
        ]);
        Case {
            parts,
            _type: PhantomData,
        }
    }
}

/// A `CASE` expression comparing against values of type `O` (`Bool` for a
/// searched `CASE`) and producing values of type `T`; NULL without `ELSE`.
pub struct Case<O, T, N, S> {
    parts: Vec<Node>,
    _type: PhantomData<fn() -> (O, T, N, S)>,
}

impl<O, T: SqlType, N: Nullability, S> Case<O, T, N, S> {
    /// Further `WHEN condition THEN value`.
    pub fn when<C: Arg<O>, V: Arg<T>>(
        mut self,
        condition: C,
        value: V,
    ) -> Case<O, T, N::Or<V::Null>, (S, (C::Scope, V::Scope))> {
        self.parts.extend([
            Node::Keyword("WHEN"),
            condition.into_arg(),
            Node::Keyword("THEN"),
            value.into_arg(),
        ]);
        Case {
            parts: self.parts,
            _type: PhantomData,
        }
    }

    /// `ELSE value END`
    pub fn else_<V: Arg<T>>(mut self, value: V) -> Expr<T, N::Or<V::Null>, (S, V::Scope)> {
        self.parts.extend([
            Node::Keyword("ELSE"),
            value.into_arg(),
            Node::Keyword("END"),
        ]);
        Expr::new(Node::Seq(self.parts))
    }
}

impl<O, T: SqlType, N: Nullability, S> Expression for Case<O, T, N, S> {
    type Sql = T;
    type Null = Nullable;
    type Scope = S;
    fn into_node(mut self) -> Node {
        self.parts.push(Node::Keyword("END"));
        Node::Seq(self.parts)
    }
}
/// Implements Rust operators for an expression type.
macro_rules! rust_operators {
    ($($ty:ident<$($p:ident),*>),*) => {$(
        rust_operators!(@binary $ty [$($p),*], Add add Plus, Sub sub Minus, Mul mul Mul, Div div Div,
            Rem rem Rem, BitAnd bitand BitAnd, BitOr bitor BitOr, BitXor bitxor Xor, Shl shl Shl,
            Shr shr Shr);

        impl<$($p),*> std::ops::Neg for $ty<$($p),*>
        where
            Self: $crate::Expression,
            <Self as $crate::Expression>::Sql: $crate::Prefix<$crate::op::Neg>,
        {
            type Output = $crate::Unary<Self, $crate::op::Neg>;
            fn neg(self) -> Self::Output {
                $crate::expr::unary(self)
            }
        }

        impl<$($p),*> std::ops::Not for $ty<$($p),*>
        where
            Self: $crate::Expression,
            <Self as $crate::Expression>::Sql: $crate::Prefix<$crate::op::Not>,
        {
            type Output = $crate::Unary<Self, $crate::op::Not>;
            fn not(self) -> Self::Output {
                $crate::expr::unary(self)
            }
        }
    )*};
    (@binary $ty:ident $generics:tt, $($trait:ident $method:ident $op:ident),*) => {$(
        rust_operators!(@one $ty $generics $trait $method $op);
    )*};
    (@one $ty:ident [$($p:ident),*] $trait:ident $method:ident $op:ident) => {
        impl<$($p,)* Rhs: $crate::Expression> std::ops::$trait<Rhs> for $ty<$($p),*>
        where
            Self: $crate::Expression,
            <Self as $crate::Expression>::Sql: $crate::Operator<$crate::op::$op, Rhs::Sql>,
        {
            type Output = $crate::Binary<Self, $crate::op::$op, Rhs>;
            fn $method(self, right: Rhs) -> Self::Output {
                $crate::expr::binary(self, right)
            }
        }
    };
}

pub(crate) use rust_operators;

rust_operators!(Expr<T, N, S>, Column<T, N, Q>, Case<O, T, N, S>);

impl<T: SqlType, N: Nullability, S> ExpressionMethods for Expr<T, N, S> {}
impl<T: SqlType, N: Nullability, Q: Qualifier> ExpressionMethods for Column<T, N, Q> {}
impl<O, T: SqlType, N: Nullability, S> ExpressionMethods for Case<O, T, N, S> {}
