use crate::schema::{Mood, PositiveInt, PriceTag, type_samples};
use surus::prelude::*;
use surus::types::{
    Array, Bit, Bool, Bpchar, Bytea, Char, Cidr, Circle, Date, Float4, Float8, Inet, Int2, Int4,
    Int8, Interval, Json, Jsonb, Jsonpath, Line, Lseg, Macaddr, Macaddr8, Money, Multirange,
    Nullable, Numeric, Path, PgBox, Point, Polygon, Range, Text, Time, Timestamp, Timestamptz,
    Timetz, Tsquery, Tsvector, Uuid, Varbit, Varchar, Xml,
};
use surus::{Query, query};

pub fn bool() -> Query<(Bool,), (Nullable<Bool>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_bool(value))
            .returning(|s| s.v_bool)
    })
}

pub fn int2() -> Query<(Int2,), (Nullable<Int2>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_int2(value))
            .returning(|s| s.v_int2)
    })
}

pub fn int4() -> Query<(Int4,), (Nullable<Int4>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_int4(value))
            .returning(|s| s.v_int4)
    })
}

pub fn int8() -> Query<(Int8,), (Nullable<Int8>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_int8(value))
            .returning(|s| s.v_int8)
    })
}

pub fn numeric() -> Query<(Numeric,), (Nullable<Numeric>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_numeric(value))
            .returning(|s| s.v_numeric)
    })
}

pub fn float4() -> Query<(Float4,), (Nullable<Float4>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_float4(value))
            .returning(|s| s.v_float4)
    })
}

pub fn float8() -> Query<(Float8,), (Nullable<Float8>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_float8(value))
            .returning(|s| s.v_float8)
    })
}

pub fn money() -> Query<(Money,), (Nullable<Money>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_money(value))
            .returning(|s| s.v_money)
    })
}

pub fn text() -> Query<(Text,), (Nullable<Text>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_text(value))
            .returning(|s| s.v_text)
    })
}

pub fn varchar() -> Query<(Varchar,), (Nullable<Varchar>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_varchar(value))
            .returning(|s| s.v_varchar)
    })
}

pub fn bpchar() -> Query<(Bpchar,), (Nullable<Bpchar>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_bpchar(value))
            .returning(|s| s.v_bpchar)
    })
}

pub fn char() -> Query<(Char,), (Nullable<Char>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_char(value))
            .returning(|s| s.v_char)
    })
}

pub fn date() -> Query<(Date,), (Nullable<Date>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_date(value))
            .returning(|s| s.v_date)
    })
}

pub fn time() -> Query<(Time,), (Nullable<Time>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_time(value))
            .returning(|s| s.v_time)
    })
}

pub fn timetz() -> Query<(Timetz,), (Nullable<Timetz>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_timetz(value))
            .returning(|s| s.v_timetz)
    })
}

pub fn timestamp() -> Query<(Timestamp,), (Nullable<Timestamp>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_timestamp(value))
            .returning(|s| s.v_timestamp)
    })
}

pub fn timestamptz() -> Query<(Timestamptz,), (Nullable<Timestamptz>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_timestamptz(value))
            .returning(|s| s.v_timestamptz)
    })
}

pub fn interval() -> Query<(Interval,), (Nullable<Interval>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_interval(value))
            .returning(|s| s.v_interval)
    })
}

pub fn bytea() -> Query<(Bytea,), (Nullable<Bytea>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_bytea(value))
            .returning(|s| s.v_bytea)
    })
}

pub fn uuid() -> Query<(Uuid,), (Nullable<Uuid>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_uuid(value))
            .returning(|s| s.v_uuid)
    })
}

pub fn json() -> Query<(Json,), (Nullable<Json>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_json(value))
            .returning(|s| s.v_json)
    })
}

pub fn jsonb() -> Query<(Jsonb,), (Nullable<Jsonb>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_jsonb(value))
            .returning(|s| s.v_jsonb)
    })
}

pub fn jsonpath() -> Query<(Jsonpath,), (Nullable<Jsonpath>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_jsonpath(value))
            .returning(|s| s.v_jsonpath)
    })
}

pub fn inet() -> Query<(Inet,), (Nullable<Inet>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_inet(value))
            .returning(|s| s.v_inet)
    })
}

pub fn cidr() -> Query<(Cidr,), (Nullable<Cidr>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_cidr(value))
            .returning(|s| s.v_cidr)
    })
}

pub fn macaddr() -> Query<(Macaddr,), (Nullable<Macaddr>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_macaddr(value))
            .returning(|s| s.v_macaddr)
    })
}

pub fn macaddr8() -> Query<(Macaddr8,), (Nullable<Macaddr8>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_macaddr8(value))
            .returning(|s| s.v_macaddr8)
    })
}

pub fn bit() -> Query<(Bit,), (Nullable<Bit>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_bit(value))
            .returning(|s| s.v_bit)
    })
}

pub fn varbit() -> Query<(Varbit,), (Nullable<Varbit>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_varbit(value))
            .returning(|s| s.v_varbit)
    })
}

pub fn point() -> Query<(Point,), (Nullable<Point>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_point(value))
            .returning(|s| s.v_point)
    })
}

pub fn line() -> Query<(Line,), (Nullable<Line>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_line(value))
            .returning(|s| s.v_line)
    })
}

pub fn lseg() -> Query<(Lseg,), (Nullable<Lseg>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_lseg(value))
            .returning(|s| s.v_lseg)
    })
}

pub fn r#box() -> Query<(PgBox,), (Nullable<PgBox>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_box(value))
            .returning(|s| s.v_box)
    })
}

pub fn circle() -> Query<(Circle,), (Nullable<Circle>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_circle(value))
            .returning(|s| s.v_circle)
    })
}

pub fn path() -> Query<(Path,), (Nullable<Path>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_path(value))
            .returning(|s| s.v_path)
    })
}

pub fn polygon() -> Query<(Polygon,), (Nullable<Polygon>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_polygon(value))
            .returning(|s| s.v_polygon)
    })
}

pub fn tsvector() -> Query<(Tsvector,), (Nullable<Tsvector>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_tsvector(value))
            .returning(|s| s.v_tsvector)
    })
}

pub fn tsquery() -> Query<(Tsquery,), (Nullable<Tsquery>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_tsquery(value))
            .returning(|s| s.v_tsquery)
    })
}

pub fn xml() -> Query<(Xml,), (Nullable<Xml>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_xml(value))
            .returning(|s| s.v_xml)
    })
}

pub fn int4_array() -> Query<(Array<Int4>,), (Nullable<Array<Int4>>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_int4_array(value))
            .returning(|s| s.v_int4_array)
    })
}

pub fn text_array() -> Query<(Array<Text>,), (Nullable<Array<Text>>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_text_array(value))
            .returning(|s| s.v_text_array)
    })
}

pub fn int4range() -> Query<(Range<Int4>,), (Nullable<Range<Int4>>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_int4range(value))
            .returning(|s| s.v_int4range)
    })
}

pub fn tsrange() -> Query<(Range<Timestamp>,), (Nullable<Range<Timestamp>>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_tsrange(value))
            .returning(|s| s.v_tsrange)
    })
}

pub fn int4multirange() -> Query<(Multirange<Int4>,), (Nullable<Multirange<Int4>>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_int4multirange(value))
            .returning(|s| s.v_int4multirange)
    })
}

pub fn mood() -> Query<(Mood,), (Nullable<Mood>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_mood(value))
            .returning(|s| s.v_mood)
    })
}

pub fn positive_int() -> Query<(PositiveInt,), (Nullable<PositiveInt>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_positive_int(value))
            .returning(|s| s.v_positive_int)
    })
}

pub fn price_tag() -> Query<(PriceTag,), (Nullable<PriceTag>,)> {
    query(|(value,)| {
        type_samples
            .insert(|s| s.v_price_tag(value))
            .returning(|s| s.v_price_tag)
    })
}
