//! PostgreSQL operators, one trait per operator.
//!
//! `L: Contains<R>` means `L @> R` is defined, with result type `Output`.
//! Implementations for built-in types are generated from `pg_operator`; the
//! polymorphic operators over arrays, ranges, enums and records are written
//! here as generic implementations.

use crate::types::*;

macro_rules! traits {
    (binary: $($binary:ident)*; prefix: $($prefix:ident)*;) => {
        $(
            pub trait $binary<R> {
                type Output: PgType<Null = NotNull>;
                const OP: &'static str;
            }
        )*
        $(
            pub trait $prefix {
                type Output: PgType<Null = NotNull>;
                const OP: &'static str;
            }
        )*
    };
}

traits! {
    binary: Above Adjacent Below BitAnd BitOr BitXor ClosestPoint Concat ContainedBy Contains
        Difference Distance Div Eq FollowedBy Ge Get GetPath GetPathText GetText Gt HasAllKeys
        HasAnyKey HasKey HorizontallyAligned Ilike ImageEq ImageGe ImageGt ImageLe ImageLt ImageNe
        Intersection Intersects Iregex IsParallel IsPerpendicular Le LeftOf Like Lt Matches
        MatchesAlt Minus Mul Ne NotIlike NotIregex NotLike NotRegex Overabove Overbelow Overlaps
        Overleft Overright PatGe PatGt PatLe PatLt PathExists Plus Pow Regex Rem Remove RemovePath
        RightOf SameAs Shl Shr StartsWith StrictlyAbove StrictlyBelow SubnetGe SubnetGt SubnetLe
        SubnetLt TsAnd TsOr Union VerticallyAligned;
    prefix: Abs BitNot Cbrt Center IsHorizontal IsVertical Length Neg Negate Npoints Sqrt UnaryPlus;
}

macro_rules! binary {
    ($trait:ident $op:literal: $($left:ty, $right:ty => $output:ty;)*) => {$(
        impl $trait<$right> for $left {
            type Output = $output;
            const OP: &'static str = $op;
        }
    )*};
}

macro_rules! prefix {
    ($trait:ident $op:literal: $($operand:ty => $output:ty;)*) => {$(
        impl $trait for $operand {
            type Output = $output;
            const OP: &'static str = $op;
        }
    )*};
}

mod catalog;

macro_rules! generic {
    ($params:tt $($trait:ident $op:literal: $left:ty, $right:ty => $output:ty;)*) => {$(
        generic!(@one $params $trait $op $left, $right => $output);
    )*};
    (@one [$($param:tt)*] $trait:ident $op:literal $left:ty, $right:ty => $output:ty) => {
        impl<$($param)*> $trait<$right> for $left {
            type Output = $output;
            const OP: &'static str = $op;
        }
    };
}

generic! { [T: Element]
    Eq "=": Array<T>, Array<T> => Bool;
    Ne "<>": Array<T>, Array<T> => Bool;
    Lt "<": Array<T>, Array<T> => Bool;
    Le "<=": Array<T>, Array<T> => Bool;
    Gt ">": Array<T>, Array<T> => Bool;
    Ge ">=": Array<T>, Array<T> => Bool;
    Overlaps "&&": Array<T>, Array<T> => Bool;
    Contains "@>": Array<T>, Array<T> => Bool;
    ContainedBy "<@": Array<T>, Array<T> => Bool;
    Concat "||": Array<T>, Array<T> => Array<T>;
    Concat "||": Array<T>, T => Array<T>;
    Concat "||": T, Array<T> => Array<T>;
}

generic! { [T: Comparable]
    Eq "=": T, T => Bool;
    Ne "<>": T, T => Bool;
    Lt "<": T, T => Bool;
    Le "<=": T, T => Bool;
    Gt ">": T, T => Bool;
    Ge ">=": T, T => Bool;
}

generic! { [T: Composite]
    ImageEq "*=": T, T => Bool;
    ImageNe "*<>": T, T => Bool;
    ImageLt "*<": T, T => Bool;
    ImageLe "*<=": T, T => Bool;
    ImageGt "*>": T, T => Bool;
    ImageGe "*>=": T, T => Bool;
}

macro_rules! ranges {
    ($($left:ident $right:ident)*) => {$(
        generic! { [T: RangeSubtype]
            Overlaps "&&": $left<T>, $right<T> => Bool;
            Overleft "&<": $left<T>, $right<T> => Bool;
            Overright "&>": $left<T>, $right<T> => Bool;
            LeftOf "<<": $left<T>, $right<T> => Bool;
            RightOf ">>": $left<T>, $right<T> => Bool;
            Adjacent "-|-": $left<T>, $right<T> => Bool;
            Contains "@>": $left<T>, $right<T> => Bool;
            ContainedBy "<@": $left<T>, $right<T> => Bool;
        }
    )*};
}

ranges!(Range Range Range Multirange Multirange Range Multirange Multirange);

macro_rules! range_sets {
    ($($kind:ident)*) => {$(
        generic! { [T: RangeSubtype]
            Eq "=": $kind<T>, $kind<T> => Bool;
            Ne "<>": $kind<T>, $kind<T> => Bool;
            Lt "<": $kind<T>, $kind<T> => Bool;
            Le "<=": $kind<T>, $kind<T> => Bool;
            Gt ">": $kind<T>, $kind<T> => Bool;
            Ge ">=": $kind<T>, $kind<T> => Bool;
            Contains "@>": $kind<T>, T => Bool;
            ContainedBy "<@": T, $kind<T> => Bool;
            Union "+": $kind<T>, $kind<T> => $kind<T>;
            Intersection "*": $kind<T>, $kind<T> => $kind<T>;
            Difference "-": $kind<T>, $kind<T> => $kind<T>;
        }
    )*};
}

range_sets!(Range Multirange);

/// Declares the operator methods of `Expression`.
macro_rules! methods {
    (
        binary: $($method:ident $trait:ident $doc:literal)*;
        prefix: $($pmethod:ident $ptrait:ident $pdoc:literal)*;
    ) => {
        $(
            #[doc = concat!("The ", $doc, " operator.")]
            fn $method<R: Operand>(
                self,
                rhs: R,
            ) -> Expr<Output<Self, R, <Base<Self> as ops::$trait<Base<R>>>::Output>>
            where
                Base<Self>: ops::$trait<Base<R>>,
            {
                let op = <Base<Self> as ops::$trait<Base<R>>>::OP;
                Expr::new(Node::binary(op, self.into_node(), rhs.into_node()))
            }
        )*
        $(
            #[doc = concat!("The prefix ", $pdoc, " operator.")]
            fn $pmethod(self) -> Expr<Like<<Base<Self> as ops::$ptrait>::Output, Self::Type>>
            where
                Base<Self>: ops::$ptrait,
            {
                let op = <Base<Self> as ops::$ptrait>::OP;
                Expr::new(Node::Prefix(op, Box::new(self.into_node())))
            }
        )*
    };
}

pub(crate) use methods;

macro_rules! text_concat {
    ($($kind:ident)*) => {$(
        generic! { [T: RangeSubtype]
            Concat "||": Text, $kind<T> => Text;
            Concat "||": $kind<T>, Text => Text;
        }
    )*};
}

text_concat!(Range Multirange);
