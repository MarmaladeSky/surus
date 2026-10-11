use crate::schema::{Mood, PositiveInt, PriceTag};
use crate::support::placeholder;
use surus::Query;
use surus::types::{
    Array, Bit, Bool, Bpchar, Bytea, Char, Cidr, Circle, Date, Float4, Float8, Inet, Int2, Int4,
    Int8, Interval, Json, Jsonb, Jsonpath, Line, Lseg, Macaddr, Macaddr8, Money, Multirange,
    Nullable, Numeric, Path, PgBox, Point, Polygon, Range, Text, Time, Timestamp, Timestamptz,
    Timetz, Tsquery, Tsvector, Uuid, Varbit, Varchar, Xml,
};

pub fn bool() -> Query<(Bool,), (Nullable<Bool>,)> {
    placeholder()
}

pub fn int2() -> Query<(Int2,), (Nullable<Int2>,)> {
    placeholder()
}

pub fn int4() -> Query<(Int4,), (Nullable<Int4>,)> {
    placeholder()
}

pub fn int8() -> Query<(Int8,), (Nullable<Int8>,)> {
    placeholder()
}

pub fn numeric() -> Query<(Numeric,), (Nullable<Numeric>,)> {
    placeholder()
}

pub fn float4() -> Query<(Float4,), (Nullable<Float4>,)> {
    placeholder()
}

pub fn float8() -> Query<(Float8,), (Nullable<Float8>,)> {
    placeholder()
}

pub fn money() -> Query<(Money,), (Nullable<Money>,)> {
    placeholder()
}

pub fn text() -> Query<(Text,), (Nullable<Text>,)> {
    placeholder()
}

pub fn varchar() -> Query<(Varchar,), (Nullable<Varchar>,)> {
    placeholder()
}

pub fn bpchar() -> Query<(Bpchar,), (Nullable<Bpchar>,)> {
    placeholder()
}

pub fn char() -> Query<(Char,), (Nullable<Char>,)> {
    placeholder()
}

pub fn date() -> Query<(Date,), (Nullable<Date>,)> {
    placeholder()
}

pub fn time() -> Query<(Time,), (Nullable<Time>,)> {
    placeholder()
}

pub fn timetz() -> Query<(Timetz,), (Nullable<Timetz>,)> {
    placeholder()
}

pub fn timestamp() -> Query<(Timestamp,), (Nullable<Timestamp>,)> {
    placeholder()
}

pub fn timestamptz() -> Query<(Timestamptz,), (Nullable<Timestamptz>,)> {
    placeholder()
}

pub fn interval() -> Query<(Interval,), (Nullable<Interval>,)> {
    placeholder()
}

pub fn bytea() -> Query<(Bytea,), (Nullable<Bytea>,)> {
    placeholder()
}

pub fn uuid() -> Query<(Uuid,), (Nullable<Uuid>,)> {
    placeholder()
}

pub fn json() -> Query<(Json,), (Nullable<Json>,)> {
    placeholder()
}

pub fn jsonb() -> Query<(Jsonb,), (Nullable<Jsonb>,)> {
    placeholder()
}

pub fn jsonpath() -> Query<(Jsonpath,), (Nullable<Jsonpath>,)> {
    placeholder()
}

pub fn inet() -> Query<(Inet,), (Nullable<Inet>,)> {
    placeholder()
}

pub fn cidr() -> Query<(Cidr,), (Nullable<Cidr>,)> {
    placeholder()
}

pub fn macaddr() -> Query<(Macaddr,), (Nullable<Macaddr>,)> {
    placeholder()
}

pub fn macaddr8() -> Query<(Macaddr8,), (Nullable<Macaddr8>,)> {
    placeholder()
}

pub fn bit() -> Query<(Bit,), (Nullable<Bit>,)> {
    placeholder()
}

pub fn varbit() -> Query<(Varbit,), (Nullable<Varbit>,)> {
    placeholder()
}

pub fn point() -> Query<(Point,), (Nullable<Point>,)> {
    placeholder()
}

pub fn line() -> Query<(Line,), (Nullable<Line>,)> {
    placeholder()
}

pub fn lseg() -> Query<(Lseg,), (Nullable<Lseg>,)> {
    placeholder()
}

pub fn r#box() -> Query<(PgBox,), (Nullable<PgBox>,)> {
    placeholder()
}

pub fn circle() -> Query<(Circle,), (Nullable<Circle>,)> {
    placeholder()
}

pub fn path() -> Query<(Path,), (Nullable<Path>,)> {
    placeholder()
}

pub fn polygon() -> Query<(Polygon,), (Nullable<Polygon>,)> {
    placeholder()
}

pub fn tsvector() -> Query<(Tsvector,), (Nullable<Tsvector>,)> {
    placeholder()
}

pub fn tsquery() -> Query<(Tsquery,), (Nullable<Tsquery>,)> {
    placeholder()
}

pub fn xml() -> Query<(Xml,), (Nullable<Xml>,)> {
    placeholder()
}

pub fn int4_array() -> Query<(Array<Int4>,), (Nullable<Array<Int4>>,)> {
    placeholder()
}

pub fn text_array() -> Query<(Array<Text>,), (Nullable<Array<Text>>,)> {
    placeholder()
}

pub fn int4range() -> Query<(Range<Int4>,), (Nullable<Range<Int4>>,)> {
    placeholder()
}

pub fn tsrange() -> Query<(Range<Timestamp>,), (Nullable<Range<Timestamp>>,)> {
    placeholder()
}

pub fn int4multirange() -> Query<(Multirange<Int4>,), (Nullable<Multirange<Int4>>,)> {
    placeholder()
}

pub fn mood() -> Query<(Mood,), (Nullable<Mood>,)> {
    placeholder()
}

pub fn positive_int() -> Query<(PositiveInt,), (Nullable<PositiveInt>,)> {
    placeholder()
}

pub fn price_tag() -> Query<(PriceTag,), (Nullable<PriceTag>,)> {
    placeholder()
}
