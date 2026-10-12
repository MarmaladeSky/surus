//! Implementations for tuples of every arity up to 32.

use crate::ast::{Item as ItemNode, Node, Order as OrderNode, Rel};
use crate::expr::{
    Aliased, Assignment, Assignments, Column, InList, Operand, Operands, RowCompare,
};
use crate::query::{
    Append, Exprs, Grouping, KeySet, KeySets, OrderBy, Projection, Source, Sources, UnionRow,
    key_index,
};
use crate::schema::{Expand, Star};
use crate::types::*;
use crate::{Arg, Bind, Param, Params, Row, ToPg, ops};
use std::marker::PhantomData;

const NAMES: [&str; 32] = [
    "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "c10", "c11", "c12", "c13", "c14", "c15",
    "c16", "c17", "c18", "c19", "c20", "c21", "c22", "c23", "c24", "c25", "c26", "c27", "c28",
    "c29", "c30", "c31", "c32",
];

/// A single-column SELECT list item.
pub trait Item {
    type Type: PgType;
    #[doc(hidden)]
    fn item(self) -> ItemNode;
}

impl<E: Operand> Item for E {
    type Type = E::Type;
    fn item(self) -> ItemNode {
        ItemNode {
            node: self.into_node(),
            alias: None,
        }
    }
}

impl<E: Operand> Item for Aliased<E> {
    type Type = E::Type;
    fn item(self) -> ItemNode {
        ItemNode {
            node: self.expr.into_node(),
            alias: Some(self.name),
        }
    }
}

/// Type-level tuple concatenation, for `source.star()` in a SELECT list.
pub trait Concat<R> {
    type Out;
}

impl<L> Concat<()> for L {
    type Out = L;
}

/// Appends a type to a tuple type.
pub trait Push<T> {
    type Out;
}

impl<T> Push<T> for () {
    type Out = (T,);
}

impl Row for () {
    type Columns = ();
    const LEN: usize = 0;
    fn types() -> Vec<String> {
        Vec::new()
    }
    fn columns_by(_: &dyn Fn(usize) -> Rel) {}
}

impl Source for () {
    type Nullable = ();
    fn nullable(&self) {}
}

impl Params for () {
    type Handles = ();
    const LEN: usize = 0;
    fn handles() {}
}

impl Bind<()> for () {
    fn args(self) -> Vec<Arg> {
        Vec::new()
    }
}

impl KeySet for () {
    fn set(self) -> Vec<usize> {
        Vec::new()
    }
}

impl<S0: Source> Sources for (S0,) {
    type View = S0;
    fn view(&self) -> S0 {
        self.0.clone()
    }
}

macro_rules! nullity {
    ($first:ty $(, $rest:ty)*) => { nullity!(@ $first; $($rest),*) };
    (@ $acc:ty; ) => { $acc };
    (@ $acc:ty; $next:ty $(, $rest:ty)*) => {
        nullity!(@ <$acc as Nullity>::Or<$next>; $($rest),*)
    };
}

macro_rules! tuple {
    ($($t:ident $u:ident $i:tt),*) => {
        impl<$($t: PgType),*> Params for ($($t,)*) {
            type Handles = ($(Param<$t, $i>,)*);
            const LEN: usize = [$($i),*].len();
            fn handles() -> Self::Handles {
                ($(Param::<$t, $i>(PhantomData),)*)
            }
        }

        impl<$($t: PgType, $u: ToPg<$t>),*> Bind<($($t,)*)> for ($($u,)*) {
            fn args(self) -> Vec<Arg> {
                vec![$(Arg { ty: $t::name(), value: self.$i.to_pg() }),*]
            }
        }

        impl<$($t: PgType),*> Row for ($($t,)*) {
            type Columns = ($(Column<$t>,)*);
            const LEN: usize = [$($i),*].len();
            fn types() -> Vec<String> {
                vec![$($t::name()),*]
            }
            fn columns_by(rel: &dyn Fn(usize) -> Rel) -> Self::Columns {
                ($(Column::new(rel($i), NAMES[$i]),)*)
            }
        }

        impl<$($t: Source),*> Source for ($($t,)*) {
            type Nullable = ($($t::Nullable,)*);
            fn nullable(&self) -> Self::Nullable {
                ($(self.$i.nullable(),)*)
            }
        }

        impl<$($t: Operand),*> Operands for ($($t,)*) {
            type Types = ($($t::Type,)*);
            fn into_nodes(self) -> Vec<Node> {
                vec![$(self.$i.into_node()),*]
            }
        }

        impl<$($t: PgType, $u: PgType),*> RowCompare<($($u,)*)> for ($($t,)*)
        where
            $($t::Base: ops::Eq<$u::Base>,)*
        {
        }

        impl<$($t: PgType, $u: PgType),*> UnionRow<($($t,)*)> for ($($u,)*)
        where
            $($u::Base: SameType<$t::Base>,)*
        {
            type Out = ($(Strict<$t::Base, $t, $u>,)*);
        }

        impl<T, $($t: Operand),*> InList<T> for ($($t,)*)
        where
            $(T: ops::Eq<<$t::Type as PgType>::Base>,)*
        {
            type Null = nullity!($(<$t::Type as PgType>::Null),*);
            fn into_list(self) -> Node {
                Node::List(vec![$(self.$i.into_node()),*])
            }
        }

        impl<$($t: Item),*> Projection for ($($t,)*) {
            type Row = ($($t::Type,)*);
            fn items(self, items: &mut Vec<ItemNode>) {
                $(items.push(self.$i.item());)*
            }
        }

        impl<R: Row + Concat<($($t::Type,)*)>, $($t: Item),*> Projection for (Star<R>, $($t,)*)
        where
            R::Out: Row,
        {
            type Row = R::Out;
            #[allow(non_snake_case)]
            fn items(self, items: &mut Vec<ItemNode>) {
                let (head, $($t,)*) = self;
                head.items(items);
                ($($t,)*).items(items);
            }
        }

        impl<R: Row + Concat<($($t::Type,)*)>, $($t: Item),*> Projection for (Expand<R>, $($t,)*)
        where
            R::Out: Row,
        {
            type Row = R::Out;
            #[allow(non_snake_case)]
            fn items(self, items: &mut Vec<ItemNode>) {
                let (head, $($t,)*) = self;
                head.items(items);
                ($($t,)*).items(items);
            }
        }

        impl<$($t: OrderBy),*> OrderBy for ($($t,)*) {
            fn orders(self, orders: &mut Vec<OrderNode>) {
                $(self.$i.orders(orders);)*
            }
        }

        impl<$($t: Operand),*> Exprs for ($($t,)*) {
            fn into_nodes(self) -> Vec<Node> {
                vec![$(self.$i.into_node()),*]
            }
        }

        impl<$($t: Operand),*> Grouping for ($($t,)*) {
            type Scope<S: Sources> = S;
            fn into_group<S: Sources>(self, id: u64, scope: S) -> (crate::ast::Group, S) {
                let group = crate::ast::Group {
                    id,
                    kind: "",
                    keys: vec![$(self.$i.into_node()),*],
                    sets: Vec::new(),
                };
                (group, scope)
            }
        }

        impl<$($t: PgType),*> KeySet for ($(Column<$t>,)*) {
            fn set(self) -> Vec<usize> {
                vec![$(key_index(&self.$i)),*]
            }
        }

        impl<$($t: KeySet),*> KeySets for ($($t,)*) {
            fn sets(self) -> Vec<Vec<usize>> {
                vec![$(self.$i.set()),*]
            }
        }

        impl<$($t: Assignments),*> Assignments for ($($t,)*) {
            fn into_assignments(self) -> Vec<Assignment> {
                let mut all = Vec::new();
                $(all.extend(self.$i.into_assignments());)*
                all
            }
        }
    };
}

/// Implementations that relate a tuple to the tuple with one more element.
macro_rules! grow {
    ($($t:ident $i:tt,)* ; $last:ident) => {
        impl<$($t,)* $last> Push<$last> for ($($t,)*) {
            type Out = ($($t,)* $last,);
        }

        impl<$($t: Source,)* $last: Source> Append<$last> for ($($t,)*) {
            type Out = ($($t,)* $last,);
            fn append(self, source: $last) -> Self::Out {
                ($(self.$i,)* source,)
            }
        }

        impl<$($t: Source,)* $last: Source> Sources for ($($t,)* $last,) {
            type View = Self;
            fn view(&self) -> Self {
                self.clone()
            }
        }

        impl<L: Push<$last>, $last, $($t),*> Concat<($last, $($t,)*)> for L
        where
            L::Out: Concat<($($t,)*)>,
        {
            type Out = <L::Out as Concat<($($t,)*)>>::Out;
        }
    };
}

macro_rules! tuples {
    ([$($done:tt)*]) => {};
    ([$($done:tt)*] $t:ident $u:ident $i:tt $(, $($rest:tt)*)?) => {
        tuple!($($done)* $t $u $i);
        tuples!([$($done)* $t $u $i,] $($($rest)*)?);
    };
}

tuples!([]
    T0 U0 0, T1 U1 1, T2 U2 2, T3 U3 3, T4 U4 4, T5 U5 5, T6 U6 6, T7 U7 7,
    T8 U8 8, T9 U9 9, T10 U10 10, T11 U11 11, T12 U12 12, T13 U13 13, T14 U14 14, T15 U15 15,
    T16 U16 16, T17 U17 17, T18 U18 18, T19 U19 19, T20 U20 20, T21 U21 21, T22 U22 22, T23 U23 23,
    T24 U24 24, T25 U25 25, T26 U26 26, T27 U27 27, T28 U28 28, T29 U29 29, T30 U30 30, T31 U31 31
);

macro_rules! grows {
    ([$($done:tt)*] $t:ident $i:tt $(, $($rest:tt)*)?) => {
        grow!($($done)* ; $t);
        grows!([$($done)* $t $i,] $($($rest)*)?);
    };
    ([$($done:tt)*]) => {};
}

grows!([T0 0,]
    T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11, T12 12, T13 13, T14 14,
    T15 15, T16 16, T17 17, T18 18, T19 19, T20 20, T21 21, T22 22, T23 23, T24 24, T25 25,
    T26 26, T27 27, T28 28, T29 29, T30 30, T31 31
);

impl<L: Push<T0>, T0> Concat<(T0,)> for L {
    type Out = L::Out;
}

macro_rules! elements {
    ($first:ident $($t:ident)*) => {
        impl<$first: Operand, $($t: Operand),*> crate::Elements for ($first, $($t,)*)
        where
            <$first::Type as PgType>::Base: Element,
            $(<$t::Type as PgType>::Base: Coerce<<$first::Type as PgType>::Base>,)*
        {
            type Element = <$first::Type as PgType>::Base;
        }
    };
}

elements!(T0);
elements!(T0 T1);
elements!(T0 T1 T2);
elements!(T0 T1 T2 T3);
elements!(T0 T1 T2 T3 T4);
elements!(T0 T1 T2 T3 T4 T5);
elements!(T0 T1 T2 T3 T4 T5 T6);
elements!(T0 T1 T2 T3 T4 T5 T6 T7);

macro_rules! column_lists {
    ($(($($t:ident $u:ident $i:tt),*))*) => {$(
        impl<$($t: PgType),*> crate::ColumnList for ($(Column<$t>,)*) {
            type Types = ($($t,)*);
            fn names(&self) -> (Vec<&'static str>, Rel) {
                (vec![$(self.$i.name()),*], self.0.rel())
            }
        }

        impl<$($t: PgType, $u),*> crate::AssignRow<($($u,)*)> for ($($t,)*)
        where
            $($t: crate::Assignable<$u>,)*
        {
        }
    )*};
}

column_lists! {
    (T0 U0 0, T1 U1 1)
    (T0 U0 0, T1 U1 1, T2 U2 2)
    (T0 U0 0, T1 U1 1, T2 U2 2, T3 U3 3)
    (T0 U0 0, T1 U1 1, T2 U2 2, T3 U3 3, T4 U4 4)
    (T0 U0 0, T1 U1 1, T2 U2 2, T3 U3 3, T4 U4 4, T5 U5 5)
}
