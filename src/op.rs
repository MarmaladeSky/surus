//! Marker types for PostgreSQL operators. Which operand types each operator
//! accepts is generated from the server catalog (see `catalog.rs`).

/// The SQL spelling of a binary operator.
pub trait Symbol {
    const SQL: &'static str;
}

macro_rules! markers {
    ($($name:ident = $sql:literal),* $(,)?) => {$(
        #[doc = concat!("`", $sql, "`")]
        pub struct $name;
        impl Symbol for $name {
            const SQL: &'static str = $sql;
        }
    )*};
}

markers! {
    Eq = "=", Ne = "<>", Lt = "<", Le = "<=", Gt = ">", Ge = ">=",
    Plus = "+", Minus = "-", Mul = "*", Div = "/", Rem = "%", Pow = "^",
    BitAnd = "&", BitOr = "|", Xor = "#", Shl = "<<", Shr = ">>", Concat = "||",
    Like = "~~", ILike = "~~*", NotLike = "!~~", NotILike = "!~~*",
    Regex = "~", IRegex = "~*", NotRegex = "!~", NotIRegex = "!~*",
    Contains = "@>", ContainedBy = "<@", ContainsOrEquals = ">>=", ContainedByOrEquals = "<<=",
    Overlaps = "&&", Overleft = "&<", Overright = "&>", Overbelow = "&<|", Overabove = "|&>",
    StrictlyBelow = "<<|", StrictlyAbove = "|>>", Below = "<^", Above = ">^",
    Adjacent = "-|-", Distance = "<->", Intersects = "?#", Horizontal = "?-",
    Perpendicular = "?-|", Parallel = "?||", ClosestPoint = "##", SameAs = "~=",
    Matches = "@@", MatchesDeprecated = "@@@", StartsWith = "^@",
    Get = "->", GetText = "->>", GetPath = "#>", GetPathText = "#>>", DeletePath = "#-",
    HasKey = "?", HasAnyKey = "?|", HasAllKeys = "?&", PathExists = "@?",
    RecEq = "*=", RecNe = "*<>", RecLt = "*<", RecLe = "*<=", RecGt = "*>", RecGe = "*>=",
    PatternLt = "~<~", PatternLe = "~<=~", PatternGt = "~>~", PatternGe = "~>=~",
}

macro_rules! prefix_markers {
    ($($name:ident),* $(,)?) => {$(
        #[doc = concat!("Prefix operator ", stringify!($name), "; its spelling depends on the operand type.")]
        pub struct $name;
    )*};
}

prefix_markers!(
    Neg,
    Not,
    UnaryPlus,
    Abs,
    Sqrt,
    Cbrt,
    Length,
    Center,
    Npoints,
    IsHorizontal,
    IsVertical
);
