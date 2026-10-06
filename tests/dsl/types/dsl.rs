use crate::support::{Param, Query};
use schema::*;
use surus::*;

/// `INSERT INTO type_samples (column) VALUES ($1) RETURNING column`, with
/// the value supplied by the driver.
fn round_trip<T: SqlType, N: Nullability>(
    column: Column<T, N, TypeSamplesTable>,
    value: Param,
) -> Query {
    let statement = type_samples
        .insert((column,))
        .values((placeholder(),))
        .returning(column)
        .compile();
    Query::with_params(&statement.sql, vec![value])
}

pub fn bool(value: Param) -> Query {
    round_trip(type_samples.v_bool, value)
}

pub fn int2(value: Param) -> Query {
    round_trip(type_samples.v_int2, value)
}

pub fn int4(value: Param) -> Query {
    round_trip(type_samples.v_int4, value)
}

pub fn int8(value: Param) -> Query {
    round_trip(type_samples.v_int8, value)
}

pub fn numeric(value: Param) -> Query {
    round_trip(type_samples.v_numeric, value)
}

pub fn float4(value: Param) -> Query {
    round_trip(type_samples.v_float4, value)
}

pub fn float8(value: Param) -> Query {
    round_trip(type_samples.v_float8, value)
}

pub fn money(value: Param) -> Query {
    round_trip(type_samples.v_money, value)
}

pub fn text(value: Param) -> Query {
    round_trip(type_samples.v_text, value)
}

pub fn varchar(value: Param) -> Query {
    round_trip(type_samples.v_varchar, value)
}

pub fn bpchar(value: Param) -> Query {
    round_trip(type_samples.v_bpchar, value)
}

pub fn char(value: Param) -> Query {
    round_trip(type_samples.v_char, value)
}

pub fn date(value: Param) -> Query {
    round_trip(type_samples.v_date, value)
}

pub fn time(value: Param) -> Query {
    round_trip(type_samples.v_time, value)
}

pub fn timetz(value: Param) -> Query {
    round_trip(type_samples.v_timetz, value)
}

pub fn timestamp(value: Param) -> Query {
    round_trip(type_samples.v_timestamp, value)
}

pub fn timestamptz(value: Param) -> Query {
    round_trip(type_samples.v_timestamptz, value)
}

pub fn interval(value: Param) -> Query {
    round_trip(type_samples.v_interval, value)
}

pub fn bytea(value: Param) -> Query {
    round_trip(type_samples.v_bytea, value)
}

pub fn uuid(value: Param) -> Query {
    round_trip(type_samples.v_uuid, value)
}

pub fn json(value: Param) -> Query {
    round_trip(type_samples.v_json, value)
}

pub fn jsonb(value: Param) -> Query {
    round_trip(type_samples.v_jsonb, value)
}

pub fn jsonpath(value: Param) -> Query {
    round_trip(type_samples.v_jsonpath, value)
}

pub fn inet(value: Param) -> Query {
    round_trip(type_samples.v_inet, value)
}

pub fn cidr(value: Param) -> Query {
    round_trip(type_samples.v_cidr, value)
}

pub fn macaddr(value: Param) -> Query {
    round_trip(type_samples.v_macaddr, value)
}

pub fn macaddr8(value: Param) -> Query {
    round_trip(type_samples.v_macaddr8, value)
}

pub fn bit(value: Param) -> Query {
    round_trip(type_samples.v_bit, value)
}

pub fn varbit(value: Param) -> Query {
    round_trip(type_samples.v_varbit, value)
}

pub fn point(value: Param) -> Query {
    round_trip(type_samples.v_point, value)
}

pub fn line(value: Param) -> Query {
    round_trip(type_samples.v_line, value)
}

pub fn lseg(value: Param) -> Query {
    round_trip(type_samples.v_lseg, value)
}

pub fn r#box(value: Param) -> Query {
    round_trip(type_samples.v_box, value)
}

pub fn circle(value: Param) -> Query {
    round_trip(type_samples.v_circle, value)
}

pub fn path(value: Param) -> Query {
    round_trip(type_samples.v_path, value)
}

pub fn polygon(value: Param) -> Query {
    round_trip(type_samples.v_polygon, value)
}

pub fn tsvector(value: Param) -> Query {
    round_trip(type_samples.v_tsvector, value)
}

pub fn tsquery(value: Param) -> Query {
    round_trip(type_samples.v_tsquery, value)
}

pub fn xml(value: Param) -> Query {
    round_trip(type_samples.v_xml, value)
}

pub fn int4_array(value: Param) -> Query {
    round_trip(type_samples.v_int4_array, value)
}

pub fn text_array(value: Param) -> Query {
    round_trip(type_samples.v_text_array, value)
}

pub fn int4range(value: Param) -> Query {
    round_trip(type_samples.v_int4range, value)
}

pub fn tsrange(value: Param) -> Query {
    round_trip(type_samples.v_tsrange, value)
}

pub fn int4multirange(value: Param) -> Query {
    round_trip(type_samples.v_int4multirange, value)
}

pub fn mood(value: Param) -> Query {
    round_trip(type_samples.v_mood, value)
}

pub fn positive_int(value: Param) -> Query {
    round_trip(type_samples.v_positive_int, value)
}

pub fn price_tag(value: Param) -> Query {
    round_trip(type_samples.v_price_tag, value)
}
