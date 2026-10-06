//! PostgreSQL types, nullability, and the explicit mapping of Rust values to
//! SQL parameters.

use std::marker::PhantomData;
use std::time::Duration;

/// A PostgreSQL type known to the DSL.
pub trait SqlType: 'static {
    /// Writes the type name as used in casts.
    fn write_name(out: &mut String);
}

/// A type name usable as a cast target; may carry a type modifier.
pub trait CastTarget {
    type Sql: SqlType;
    fn write_name(out: &mut String);
}

pub(crate) fn type_name<T: SqlType>() -> String {
    let mut out = String::new();
    T::write_name(&mut out);
    out
}

/// Implements [`SqlType`] and [`CastTarget`] for a marker type.
#[doc(hidden)]
#[macro_export]
macro_rules! sql_type {
    ($name:ty = $sql:literal) => {
        impl $crate::SqlType for $name {
            fn write_name(out: &mut String) {
                out.push_str($sql)
            }
        }
        impl $crate::CastTarget for $name {
            type Sql = $name;
            fn write_name(out: &mut String) {
                out.push_str($sql)
            }
        }
    };
}

macro_rules! sql_types {
    ($($name:ident = $sql:literal),* $(,)?) => {$(
        #[doc = concat!("PostgreSQL `", $sql, "`.")]
        pub struct $name;
        sql_type!($name = $sql);
    )*};
}

sql_types! {
    Bool = "bool", Int2 = "int2", Int4 = "int4", Int8 = "int8", Numeric = "numeric",
    Float4 = "float4", Float8 = "float8", Money = "money", Text = "text", Bpchar = "bpchar",
    Char = "\"char\"", Date = "date", Time = "time", Timetz = "timetz", Timestamp = "timestamp",
    Timestamptz = "timestamptz", Interval = "interval", Bytea = "bytea", Uuid = "uuid",
    Json = "json", Jsonb = "jsonb", Jsonpath = "jsonpath", Inet = "inet", Cidr = "cidr",
    Macaddr = "macaddr", Macaddr8 = "macaddr8", Bit = "bit", Varbit = "varbit", Point = "point",
    Line = "line", Lseg = "lseg", PgBox = "box", Circle = "circle", Path = "path",
    Polygon = "polygon", Tsvector = "tsvector", Tsquery = "tsquery", Xml = "xml",
    Regtype = "regtype",
}

/// PostgreSQL `varchar`; `Varchar<N>` is the cast target `varchar(N)`.
pub struct Varchar<const N: u32 = 0>;

impl SqlType for Varchar {
    fn write_name(out: &mut String) {
        out.push_str("varchar")
    }
}

impl<const N: u32> CastTarget for Varchar<N> {
    type Sql = Varchar;
    fn write_name(out: &mut String) {
        out.push_str("varchar");
        if N > 0 {
            out.push_str(&format!("({N})"));
        }
    }
}

/// PostgreSQL array of `T`.
pub struct Array<T>(PhantomData<T>);
/// PostgreSQL range over `T`.
pub struct Range<T>(PhantomData<T>);
/// PostgreSQL multirange over `T`.
pub struct Multirange<T>(PhantomData<T>);
/// Anonymous row type with fields `R`, as built by `ROW(...)`.
pub struct Record<R>(PhantomData<R>);

/// Element types with built-in range types.
pub trait RangeElement: SqlType {
    const RANGE: &'static str;
}

macro_rules! range_elements {
    ($($t:ty = $range:literal),*) => {$(
        impl RangeElement for $t { const RANGE: &'static str = $range; }
    )*};
}

range_elements!(
    Int4 = "int4range",
    Int8 = "int8range",
    Numeric = "numrange",
    Date = "daterange",
    Timestamp = "tsrange",
    Timestamptz = "tstzrange"
);

macro_rules! generic_types {
    ($($t:ident<$p:ident: $bound:path> => |$out:ident| $write:expr),*) => {$(
        impl<$p: $bound> SqlType for $t<$p> {
            fn write_name($out: &mut String) { $write }
        }
        impl<$p: $bound> CastTarget for $t<$p> {
            type Sql = Self;
            fn write_name(out: &mut String) { <Self as SqlType>::write_name(out) }
        }
    )*};
}

generic_types! {
    Array<T: SqlType> => |out| { T::write_name(out); out.push_str("[]") },
    Range<T: RangeElement> => |out| out.push_str(T::RANGE),
    Multirange<T: RangeElement> => |out| {
        out.push_str(T::RANGE.trim_end_matches("range"));
        out.push_str("multirange")
    },
    Record<R: std::any::Any> => |out| out.push_str("record")
}

/// Marker for PostgreSQL enum types; generated schemas implement it.
pub trait Enum: SqlType {}

/// Marker for composite types; generated schemas implement it.
pub trait Composite: SqlType {
    /// The field types as a row.
    type Row;
}

/// Whether an expression may evaluate to NULL.
pub trait Nullability: 'static {
    type Or<B: Nullability>: Nullability;
    type And<B: Nullability>: Nullability;
    /// The row field for a value of type `T`: `T` or `Option<T>`.
    type Field<T: SqlType>: Field<Sql = T, Null = Self>;
}

/// The expression never evaluates to NULL.
pub struct NotNull;
/// The expression may evaluate to NULL.
pub struct Nullable;

impl Nullability for NotNull {
    type Or<B: Nullability> = B;
    type And<B: Nullability> = NotNull;
    type Field<T: SqlType> = T;
}

impl Nullability for Nullable {
    type Or<B: Nullability> = Nullable;
    type And<B: Nullability> = B;
    type Field<T: SqlType> = Option<T>;
}

/// Values of nullability `Self` may be stored where `Target` is required.
pub trait Fits<Target> {}
impl Fits<NotNull> for NotNull {}
impl Fits<Nullable> for NotNull {}
impl Fits<Nullable> for Nullable {}

/// A field of a result row: `T` for a NOT NULL value, `Option<T>` otherwise.
pub trait Field {
    type Sql: SqlType;
    type Null: Nullability;
}

impl<T: SqlType> Field for T {
    type Sql = T;
    type Null = NotNull;
}

impl<T: SqlType> Field for Option<T> {
    type Sql = T;
    type Null = Nullable;
}

/// A Rust value that can be sent as a parameter of SQL type `Sql`, encoded in
/// PostgreSQL text format.
pub trait Bind {
    type Sql: SqlType;
    type Null: Nullability;
    /// Text encoding, or `None` for NULL.
    fn encode(&self) -> Option<String>;
    /// The SQL type the parameter is declared as.
    fn param_type(&self) -> String {
        type_name::<Self::Sql>()
    }
}

macro_rules! bind_display {
    ($($t:ty => $sql:ty),*) => {$(
        impl Bind for $t {
            type Sql = $sql;
            type Null = NotNull;
            fn encode(&self) -> Option<String> { Some(self.to_string()) }
        }
    )*};
}

bind_display!(bool => Bool, i16 => Int2, i32 => Int4, i64 => Int8, f32 => Float4,
    f64 => Float8, str => Text, String => Text);

impl<T: Bind + ?Sized> Bind for &T {
    type Sql = T::Sql;
    type Null = T::Null;
    fn encode(&self) -> Option<String> {
        (**self).encode()
    }
    fn param_type(&self) -> String {
        (**self).param_type()
    }
}

impl<T: Bind<Null = NotNull>> Bind for Option<T> {
    type Sql = T::Sql;
    type Null = Nullable;
    fn encode(&self) -> Option<String> {
        self.as_ref().and_then(Bind::encode)
    }
}

impl<T: Bind<Null = NotNull>> Bind for [T] {
    type Sql = Array<T::Sql>;
    type Null = NotNull;
    fn encode(&self) -> Option<String> {
        let items: Vec<String> = self
            .iter()
            .map(|item| {
                let text = item.encode().unwrap_or_default();
                format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
            })
            .collect();
        Some(format!("{{{}}}", items.join(",")))
    }
}

impl<T: Bind<Null = NotNull>> Bind for Vec<T> {
    type Sql = Array<T::Sql>;
    type Null = NotNull;
    fn encode(&self) -> Option<String> {
        self.as_slice().encode()
    }
}

impl<T: Bind<Null = NotNull>, const N: usize> Bind for [T; N] {
    type Sql = Array<T::Sql>;
    type Null = NotNull;
    fn encode(&self) -> Option<String> {
        self.as_slice().encode()
    }
}

/// Bound as `interval`, with microsecond precision.
impl Bind for Duration {
    type Sql = Interval;
    type Null = NotNull;
    fn encode(&self) -> Option<String> {
        Some(format!("{} microseconds", self.as_micros()))
    }
}

/// An exact decimal number `mantissa * 10^-scale`, bound as `numeric`.
#[derive(Clone, Copy, Debug)]
pub struct Decimal {
    pub mantissa: i128,
    pub scale: u32,
}

impl Decimal {
    pub const fn new(mantissa: i128, scale: u32) -> Decimal {
        Decimal { mantissa, scale }
    }
}

impl Bind for Decimal {
    type Sql = Numeric;
    type Null = NotNull;
    fn encode(&self) -> Option<String> {
        let digits = format!(
            "{:0>width$}",
            self.mantissa.unsigned_abs(),
            width = self.scale as usize + 1
        );
        let (int, frac) = digits.split_at(digits.len() - self.scale as usize);
        let sign = if self.mantissa < 0 { "-" } else { "" };
        let point = if frac.is_empty() { "" } else { "." };
        Some(format!("{sign}{int}{point}{frac}"))
    }
}

/// A bit string of `len` bits taken from the low bits of `value`, bound as
/// `bit(len)`.
#[derive(Clone, Copy, Debug)]
pub struct Bits {
    pub value: u64,
    pub len: u8,
}

impl Bits {
    pub const fn new(value: u64, len: u8) -> Bits {
        Bits { value, len }
    }
}

impl Bind for Bits {
    type Sql = Bit;
    type Null = NotNull;
    fn encode(&self) -> Option<String> {
        Some(
            (0..self.len)
                .rev()
                .map(|i| if self.value >> i & 1 == 1 { '1' } else { '0' })
                .collect(),
        )
    }
    fn param_type(&self) -> String {
        format!("bit({})", self.len)
    }
}
