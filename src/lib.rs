// PostgreSQL types: zero-sized markers implementing PgType, generated from the catalog
pub mod types;

use std::marker::PhantomData;
use types::{
    Array, Bool, Element, Float4, Float8, Int2, Int4, Int8, Nullable, PgType, Text, Varchar,
};

// Values: how a Rust value becomes a PostgreSQL value of a statically known type
#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub ty: String,
    pub value: Option<String>,
}

pub trait ToPg<T: PgType> {
    fn to_pg(&self) -> Option<String>;
}

macro_rules! values {
    ($($v:ty => $t:ident)*) => {$(
        impl ToPg<$t> for $v {
            fn to_pg(&self) -> Option<String> {
                Some(self.to_string())
            }
        }
        impl IntoExpr<$t> for $v {
            type Expr = Value<$t>;
            fn into_expr(self) -> Value<$t> {
                Value::new(self)
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
    &str => Varchar
    String => Varchar
}

impl<T: PgType, V: ToPg<T> + ?Sized> ToPg<T> for &V {
    fn to_pg(&self) -> Option<String> {
        (**self).to_pg()
    }
}

impl<T: PgType, V: ToPg<T>> ToPg<Nullable<T>> for Option<V> {
    fn to_pg(&self) -> Option<String> {
        self.as_ref().and_then(ToPg::to_pg)
    }
}

impl<T: Element, V: ToPg<T>> ToPg<Array<T>> for [V] {
    fn to_pg(&self) -> Option<String> {
        let items: Vec<String> = self
            .iter()
            .map(|item| match item.to_pg() {
                Some(text) => format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\"")),
                None => "NULL".to_string(),
            })
            .collect();
        Some(format!("{{{}}}", items.join(",")))
    }
}

impl<T: Element, V: ToPg<T>> ToPg<Array<T>> for Vec<V> {
    fn to_pg(&self) -> Option<String> {
        self.as_slice().to_pg()
    }
}

// Expressions: what can appear inside a query, each with its SQL type T
pub trait Expr<T: PgType> {}

pub trait IntoExpr<T: PgType> {
    type Expr: Expr<T>;
    fn into_expr(self) -> Self::Expr;
}

impl<T: PgType, E: Expr<T>> IntoExpr<T> for E {
    type Expr = E;
    fn into_expr(self) -> E {
        self
    }
}

impl<T: PgType, V: ToPg<T>> IntoExpr<Nullable<T>> for Option<V> {
    type Expr = Value<Nullable<T>>;
    fn into_expr(self) -> Value<Nullable<T>> {
        Value::new(self)
    }
}

impl<T: Element, V: ToPg<T>> IntoExpr<Array<T>> for Vec<V> {
    type Expr = Value<Array<T>>;
    fn into_expr(self) -> Value<Array<T>> {
        Value::new(self)
    }
}

pub struct Value<T>(Arg, PhantomData<T>);

impl<T: PgType> Value<T> {
    pub fn new(value: impl ToPg<T>) -> Value<T> {
        Value(
            Arg {
                ty: T::name(),
                value: value.to_pg(),
            },
            PhantomData,
        )
    }

    pub fn arg(&self) -> &Arg {
        &self.0
    }
}

impl<T: PgType> Expr<T> for Value<T> {}

pub struct Lit<T>(Arg, PhantomData<T>);

impl<T: PgType> Lit<T> {
    pub fn arg(&self) -> &Arg {
        &self.0
    }
}

impl<T: PgType> Expr<T> for Lit<T> {}

pub fn lit<T: PgType>(value: impl ToPg<T>) -> Lit<T> {
    Lit(
        Arg {
            ty: T::name(),
            value: value.to_pg(),
        },
        PhantomData,
    )
}

pub struct Param<T, const N: usize>(PhantomData<T>);

impl<T, const N: usize> Clone for Param<T, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const N: usize> Copy for Param<T, N> {}

impl<T: PgType, const N: usize> Expr<T> for Param<T, N> {}

// Queries: compiled SQL with declared parameters P still to bind and row type R
pub trait Params {
    type Handles;
    const LEN: usize;
    fn handles() -> Self::Handles;
}

pub trait Bind<P> {
    fn args(self) -> Vec<Arg>;
}

pub trait Row {
    fn types() -> Vec<String>;
}

pub fn params<P: Params>() -> P::Handles {
    P::handles()
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

impl Row for () {
    fn types() -> Vec<String> {
        Vec::new()
    }
}

macro_rules! tuple {
    ($($t:ident $v:ident $i:tt),*) => {
        impl<$($t: PgType),*> Params for ($($t,)*) {
            type Handles = ($(Param<$t, $i>,)*);
            const LEN: usize = [$($i),*].len();
            fn handles() -> Self::Handles {
                ($(Param::<$t, $i>(PhantomData),)*)
            }
        }
        impl<$($t: PgType, $v: ToPg<$t>),*> Bind<($($t,)*)> for ($($v,)*) {
            fn args(self) -> Vec<Arg> {
                vec![$(Arg { ty: $t::name(), value: self.$i.to_pg() }),*]
            }
        }
        impl<$($t: PgType),*> Row for ($($t,)*) {
            fn types() -> Vec<String> {
                vec![$($t::name()),*]
            }
        }
    };
}

macro_rules! tuples {
    ([$($done:tt)*]) => {};
    ([$($done:tt)*] $t:ident $v:ident $i:tt $(, $($rest:tt)*)?) => {
        tuple!($($done)* $t $v $i);
        tuples!([$($done)* $t $v $i,] $($($rest)*)?);
    };
}

tuples!([]
    T0 V0 0, T1 V1 1, T2 V2 2, T3 V3 3, T4 V4 4, T5 V5 5, T6 V6 6, T7 V7 7,
    T8 V8 8, T9 V9 9, T10 V10 10, T11 V11 11, T12 V12 12, T13 V13 13, T14 V14 14, T15 V15 15,
    T16 V16 16, T17 V17 17, T18 V18 18, T19 V19 19, T20 V20 20, T21 V21 21, T22 V22 22, T23 V23 23,
    T24 V24 24, T25 V25 25, T26 V26 26, T27 V27 27, T28 V28 28, T29 V29 29, T30 V30 30, T31 V31 31
);

pub struct Query<P, R> {
    sql: String,
    args: Vec<Arg>,
    types: PhantomData<fn(P) -> R>,
}

impl<P, R> Query<P, R> {
    pub fn new(sql: String, args: Vec<Arg>) -> Query<P, R> {
        Query {
            sql,
            args,
            types: PhantomData,
        }
    }
}

impl<P: Params, R> Query<P, R> {
    pub fn bind(self, values: impl Bind<P>) -> Query<(), R> {
        let mut args = values.args();
        args.extend(self.args);
        Query::new(self.sql, args)
    }
}

impl<R> Query<(), R> {
    pub fn sql(&self) -> &str {
        &self.sql
    }

    pub fn args(&self) -> &[Arg] {
        &self.args
    }
}
