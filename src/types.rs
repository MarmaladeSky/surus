//! PostgreSQL types as zero-sized markers.
//!
//! Every SQL type is a `PgType`. Nullability is part of the type: `Nullable<T>`
//! wraps a non-null base type, so `Column<Nullable<Text>>` and `Column<Text>`
//! are distinct and the DSL propagates nullability through expressions.

mod catalog;

pub use catalog::*;
use std::marker::PhantomData;

pub trait PgType: 'static {
    /// The type without nullability.
    type Base: PgType<Null = NotNull>;
    type Null: Nullity;
    fn name() -> String;
}

/// Whether a value may be NULL, at the type level.
pub trait Nullity: 'static {
    /// Nullable when either side is nullable.
    type Or<N: Nullity>: Nullity;
    /// Nullable when both sides are nullable.
    type And<N: Nullity>: Nullity;
    /// `T`, made nullable when `Self` is `IsNull`.
    type Apply<T: PgType>: PgType;
}

pub struct NotNull;
pub struct IsNull;

impl Nullity for NotNull {
    type Or<N: Nullity> = N;
    type And<N: Nullity> = NotNull;
    type Apply<T: PgType> = T;
}

impl Nullity for IsNull {
    type Or<N: Nullity> = IsNull;
    type And<N: Nullity> = N;
    type Apply<T: PgType> = Nullable<T::Base>;
}

/// `O`, nullable when `A` or `B` is nullable: the result type of a strict
/// operator or function.
pub type Strict<O, A, B> =
    <<<A as PgType>::Null as Nullity>::Or<<B as PgType>::Null> as Nullity>::Apply<O>;

/// `O`, nullable when nullability `A` or `B` is `IsNull`.
pub type Either<A, B, O> = <<A as Nullity>::Or<B> as Nullity>::Apply<O>;

/// `O`, nullable when nullabilities `A` and `B` both are `IsNull`.
pub type Both<A, B, O> = <<A as Nullity>::And<B> as Nullity>::Apply<O>;

/// `O` with the nullability of `A`.
pub type Like<O, A> = <<A as PgType>::Null as Nullity>::Apply<O>;

pub struct Nullable<T>(PhantomData<T>);

impl<T: PgType> PgType for Nullable<T> {
    type Base = T::Base;
    type Null = IsNull;
    fn name() -> String {
        T::name()
    }
}

/// Types that can be array elements.
pub trait Element: PgType<Null = NotNull> {}

/// Types with a built-in range type.
pub trait RangeSubtype: PgType<Null = NotNull> {
    const RANGE: &'static str;
    const MULTIRANGE: &'static str;
}

/// Enums and composites, compared through the polymorphic `anyenum` and
/// `record` operators.
pub trait Comparable: PgType<Null = NotNull> {}

/// Composite types, which also have the `record` image comparison operators.
pub trait Composite: Comparable {}

/// Enum types.
pub trait Enum: Comparable {}

pub struct Array<T>(PhantomData<T>);
pub struct Range<T>(PhantomData<T>);
pub struct Multirange<T>(PhantomData<T>);

impl<T: Element> PgType for Array<T> {
    type Base = Self;
    type Null = NotNull;
    fn name() -> String {
        format!("{}[]", T::name())
    }
}

impl<T: RangeSubtype> PgType for Range<T> {
    type Base = Self;
    type Null = NotNull;
    fn name() -> String {
        T::RANGE.to_string()
    }
}

impl<T: RangeSubtype> Element for Range<T> {}

impl<T: RangeSubtype> PgType for Multirange<T> {
    type Base = Self;
    type Null = NotNull;
    fn name() -> String {
        T::MULTIRANGE.to_string()
    }
}

impl<T: RangeSubtype> Element for Multirange<T> {}

/// An anonymous row value, such as `ROW(a, b)`.
pub struct Record;

impl PgType for Record {
    type Base = Self;
    type Null = NotNull;
    fn name() -> String {
        "record".to_string()
    }
}

/// The type of an untyped `NULL`, which coerces to any type.
pub struct Unknown;

impl PgType for Unknown {
    type Base = Self;
    type Null = NotNull;
    fn name() -> String {
        "unknown".to_string()
    }
}

macro_rules! scalars {
    ($($ty:ident $name:literal)*) => {$(
        pub struct $ty;

        impl PgType for $ty {
            type Base = Self;
            type Null = NotNull;
            fn name() -> String {
                $name.to_string()
            }
        }

        impl Element for $ty {}
    )*};
}

macro_rules! ranges {
    ($($ty:ident $range:literal $multirange:literal)*) => {$(
        impl RangeSubtype for $ty {
            const RANGE: &'static str = $range;
            const MULTIRANGE: &'static str = $multirange;
        }
    )*};
}

pub(crate) use {ranges, scalars};

mod casts;

/// `CAST(Self AS T)` is valid: a `pg_cast` entry or an I/O conversion.
pub trait Cast<T> {}

/// `Self` is implicitly converted to `T` where a `T` is expected, as for
/// function arguments.
pub trait Coerce<T> {}

/// A `Self` value can be stored in a `T` column: implicit and assignment casts.
pub trait Assign<T> {}

macro_rules! markers {
    ($trait:ident: $($from:ty => $to:ty;)*) => {$(
        impl $trait<$to> for $from {}
    )*};
}

pub(crate) use markers;

impl<T: PgType<Null = NotNull>> Cast<T> for T {}
impl<T: PgType<Null = NotNull>> Coerce<T> for T {}
impl<T: PgType<Null = NotNull>> Assign<T> for T {}

impl<T: Element> Coerce<T> for Unknown {}
impl<T: Element> Coerce<Array<T>> for Unknown {}
impl<T: Element> Assign<T> for Unknown {}
impl<T: Element> Assign<Array<T>> for Unknown {}

impl<T: RangeSubtype> Cast<Multirange<T>> for Range<T> {}

macro_rules! string_casts {
    ($($string:ty)*) => {$(
        impl<T: Element> Cast<$string> for Array<T> {}
        impl<T: Element> Cast<Array<T>> for $string {}
        impl<T: RangeSubtype> Cast<$string> for Range<T> {}
        impl<T: RangeSubtype> Cast<Range<T>> for $string {}
        impl<T: RangeSubtype> Cast<$string> for Multirange<T> {}
        impl<T: RangeSubtype> Cast<Multirange<T>> for $string {}
        impl Cast<$string> for Record {}
    )*};
}

string_casts!(Text Varchar Bpchar);

/// The type of conditions: WHERE, HAVING, ON, FILTER, CASE WHEN and the
/// logical operators take `bool` expressions.
pub trait Boolean {}

impl Boolean for Bool {}

/// `Self` and `T` are the same type.
pub trait SameType<T> {}

impl<T> SameType<T> for T {}
