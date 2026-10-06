// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn bit_text() -> Query {
    t.select(t.v_bit.concat(t.v_text)).compile().into()
}

pub fn bool_text() -> Query {
    t.select(t.v_bool.concat(t.v_text)).compile().into()
}

pub fn box_text() -> Query {
    t.select(t.v_box.concat(t.v_text)).compile().into()
}

pub fn bpchar_text() -> Query {
    t.select(t.v_bpchar.concat(t.v_text)).compile().into()
}

pub fn bytea_bytea() -> Query {
    t.select(t.v_bytea.concat(t.v_bytea)).compile().into()
}

pub fn bytea_text() -> Query {
    t.select(t.v_bytea.concat(t.v_text)).compile().into()
}

pub fn char_text() -> Query {
    // Rejected at compile time: PostgreSQL reports `text || "char"` as ambiguous.
    Query::plain("SELECT NULL WHERE false")
}

pub fn cidr_text() -> Query {
    t.select(t.v_cidr.concat(t.v_text)).compile().into()
}

pub fn circle_text() -> Query {
    t.select(t.v_circle.concat(t.v_text)).compile().into()
}

pub fn date_text() -> Query {
    t.select(t.v_date.concat(t.v_text)).compile().into()
}

pub fn float4_text() -> Query {
    t.select(t.v_float4.concat(t.v_text)).compile().into()
}

pub fn float8_text() -> Query {
    t.select(t.v_float8.concat(t.v_text)).compile().into()
}

pub fn inet_text() -> Query {
    t.select(t.v_inet.concat(t.v_text)).compile().into()
}

pub fn int2_text() -> Query {
    t.select(t.v_int2.concat(t.v_text)).compile().into()
}

pub fn int4_array_int4() -> Query {
    t.select(t.v_int4_array.concat(t.v_int4)).compile().into()
}

pub fn int4_array_int4_array() -> Query {
    t.select(t.v_int4_array.concat(t.v_int4_array))
        .compile()
        .into()
}

pub fn int4_int4_array() -> Query {
    t.select(t.v_int4.concat(t.v_int4_array)).compile().into()
}

pub fn int4_text() -> Query {
    t.select(t.v_int4.concat(t.v_text)).compile().into()
}

pub fn int4multirange_text() -> Query {
    t.select(t.v_int4multirange.concat(t.v_text))
        .compile()
        .into()
}

pub fn int4range_text() -> Query {
    t.select(t.v_int4range.concat(t.v_text)).compile().into()
}

pub fn int8_text() -> Query {
    t.select(t.v_int8.concat(t.v_text)).compile().into()
}

pub fn interval_text() -> Query {
    t.select(t.v_interval.concat(t.v_text)).compile().into()
}

pub fn json_text() -> Query {
    t.select(t.v_json.concat(t.v_text)).compile().into()
}

pub fn jsonb_jsonb() -> Query {
    t.select(t.v_jsonb.concat(t.v_jsonb)).compile().into()
}

pub fn jsonb_text() -> Query {
    t.select(t.v_jsonb.concat(t.v_text)).compile().into()
}

pub fn jsonpath_text() -> Query {
    t.select(t.v_jsonpath.concat(t.v_text)).compile().into()
}

pub fn line_text() -> Query {
    t.select(t.v_line.concat(t.v_text)).compile().into()
}

pub fn lseg_text() -> Query {
    t.select(t.v_lseg.concat(t.v_text)).compile().into()
}

pub fn macaddr8_text() -> Query {
    t.select(t.v_macaddr8.concat(t.v_text)).compile().into()
}

pub fn macaddr_text() -> Query {
    t.select(t.v_macaddr.concat(t.v_text)).compile().into()
}

pub fn money_text() -> Query {
    t.select(t.v_money.concat(t.v_text)).compile().into()
}

pub fn mood_text() -> Query {
    t.select(t.v_mood.concat(t.v_text)).compile().into()
}

pub fn numeric_text() -> Query {
    t.select(t.v_numeric.concat(t.v_text)).compile().into()
}

pub fn path_text() -> Query {
    t.select(t.v_path.concat(t.v_text)).compile().into()
}

pub fn point_text() -> Query {
    t.select(t.v_point.concat(t.v_text)).compile().into()
}

pub fn polygon_text() -> Query {
    t.select(t.v_polygon.concat(t.v_text)).compile().into()
}

pub fn price_tag_text() -> Query {
    t.select(t.v_price_tag.concat(t.v_text)).compile().into()
}

pub fn text_array_text() -> Query {
    t.select(t.v_text_array.concat(t.v_text)).compile().into()
}

pub fn text_array_text_array() -> Query {
    t.select(t.v_text_array.concat(t.v_text_array))
        .compile()
        .into()
}

pub fn text_bit() -> Query {
    t.select(t.v_text.concat(t.v_bit)).compile().into()
}

pub fn text_bool() -> Query {
    t.select(t.v_text.concat(t.v_bool)).compile().into()
}

pub fn text_box() -> Query {
    t.select(t.v_text.concat(t.v_box)).compile().into()
}

pub fn text_bpchar() -> Query {
    t.select(t.v_text.concat(t.v_bpchar)).compile().into()
}

pub fn text_bytea() -> Query {
    t.select(t.v_text.concat(t.v_bytea)).compile().into()
}

pub fn text_char() -> Query {
    // Rejected at compile time: PostgreSQL reports `text || "char"` as ambiguous.
    Query::plain("SELECT NULL WHERE false")
}

pub fn text_cidr() -> Query {
    t.select(t.v_text.concat(t.v_cidr)).compile().into()
}

pub fn text_circle() -> Query {
    t.select(t.v_text.concat(t.v_circle)).compile().into()
}

pub fn text_date() -> Query {
    t.select(t.v_text.concat(t.v_date)).compile().into()
}

pub fn text_float4() -> Query {
    t.select(t.v_text.concat(t.v_float4)).compile().into()
}

pub fn text_float8() -> Query {
    t.select(t.v_text.concat(t.v_float8)).compile().into()
}

pub fn text_inet() -> Query {
    t.select(t.v_text.concat(t.v_inet)).compile().into()
}

pub fn text_int2() -> Query {
    t.select(t.v_text.concat(t.v_int2)).compile().into()
}

pub fn text_int4() -> Query {
    t.select(t.v_text.concat(t.v_int4)).compile().into()
}

pub fn text_int4multirange() -> Query {
    t.select(t.v_text.concat(t.v_int4multirange))
        .compile()
        .into()
}

pub fn text_int4range() -> Query {
    t.select(t.v_text.concat(t.v_int4range)).compile().into()
}

pub fn text_int8() -> Query {
    t.select(t.v_text.concat(t.v_int8)).compile().into()
}

pub fn text_interval() -> Query {
    t.select(t.v_text.concat(t.v_interval)).compile().into()
}

pub fn text_json() -> Query {
    t.select(t.v_text.concat(t.v_json)).compile().into()
}

pub fn text_jsonb() -> Query {
    t.select(t.v_text.concat(t.v_jsonb)).compile().into()
}

pub fn text_jsonpath() -> Query {
    t.select(t.v_text.concat(t.v_jsonpath)).compile().into()
}

pub fn text_line() -> Query {
    t.select(t.v_text.concat(t.v_line)).compile().into()
}

pub fn text_lseg() -> Query {
    t.select(t.v_text.concat(t.v_lseg)).compile().into()
}

pub fn text_macaddr() -> Query {
    t.select(t.v_text.concat(t.v_macaddr)).compile().into()
}

pub fn text_macaddr8() -> Query {
    t.select(t.v_text.concat(t.v_macaddr8)).compile().into()
}

pub fn text_money() -> Query {
    t.select(t.v_text.concat(t.v_money)).compile().into()
}

pub fn text_mood() -> Query {
    t.select(t.v_text.concat(t.v_mood)).compile().into()
}

pub fn text_numeric() -> Query {
    t.select(t.v_text.concat(t.v_numeric)).compile().into()
}

pub fn text_path() -> Query {
    t.select(t.v_text.concat(t.v_path)).compile().into()
}

pub fn text_point() -> Query {
    t.select(t.v_text.concat(t.v_point)).compile().into()
}

pub fn text_polygon() -> Query {
    t.select(t.v_text.concat(t.v_polygon)).compile().into()
}

pub fn text_price_tag() -> Query {
    t.select(t.v_text.concat(t.v_price_tag)).compile().into()
}

pub fn text_text() -> Query {
    t.select(t.v_text.concat(t.v_text)).compile().into()
}

pub fn text_text_array() -> Query {
    t.select(t.v_text.concat(t.v_text_array)).compile().into()
}

pub fn text_time() -> Query {
    t.select(t.v_text.concat(t.v_time)).compile().into()
}

pub fn text_timestamp() -> Query {
    t.select(t.v_text.concat(t.v_timestamp)).compile().into()
}

pub fn text_timestamptz() -> Query {
    t.select(t.v_text.concat(t.v_timestamptz)).compile().into()
}

pub fn text_timetz() -> Query {
    t.select(t.v_text.concat(t.v_timetz)).compile().into()
}

pub fn text_tsquery() -> Query {
    t.select(t.v_text.concat(t.v_tsquery)).compile().into()
}

pub fn text_tsrange() -> Query {
    t.select(t.v_text.concat(t.v_tsrange)).compile().into()
}

pub fn text_tsvector() -> Query {
    t.select(t.v_text.concat(t.v_tsvector)).compile().into()
}

pub fn text_uuid() -> Query {
    t.select(t.v_text.concat(t.v_uuid)).compile().into()
}

pub fn text_varbit() -> Query {
    t.select(t.v_text.concat(t.v_varbit)).compile().into()
}

pub fn text_varchar() -> Query {
    t.select(t.v_text.concat(t.v_varchar)).compile().into()
}

pub fn text_xml() -> Query {
    t.select(t.v_text.concat(t.v_xml)).compile().into()
}

pub fn time_text() -> Query {
    t.select(t.v_time.concat(t.v_text)).compile().into()
}

pub fn timestamp_text() -> Query {
    t.select(t.v_timestamp.concat(t.v_text)).compile().into()
}

pub fn timestamptz_text() -> Query {
    t.select(t.v_timestamptz.concat(t.v_text)).compile().into()
}

pub fn timetz_text() -> Query {
    t.select(t.v_timetz.concat(t.v_text)).compile().into()
}

pub fn tsquery_text() -> Query {
    t.select(t.v_tsquery.concat(t.v_text)).compile().into()
}

pub fn tsquery_tsquery() -> Query {
    t.select(t.v_tsquery.concat(t.v_tsquery)).compile().into()
}

pub fn tsrange_text() -> Query {
    t.select(t.v_tsrange.concat(t.v_text)).compile().into()
}

pub fn tsvector_text() -> Query {
    t.select(t.v_tsvector.concat(t.v_text)).compile().into()
}

pub fn tsvector_tsvector() -> Query {
    t.select(t.v_tsvector.concat(t.v_tsvector)).compile().into()
}

pub fn uuid_text() -> Query {
    t.select(t.v_uuid.concat(t.v_text)).compile().into()
}

pub fn varbit_text() -> Query {
    t.select(t.v_varbit.concat(t.v_text)).compile().into()
}

pub fn varbit_varbit() -> Query {
    t.select(t.v_varbit.concat(t.v_varbit)).compile().into()
}

pub fn varchar_text() -> Query {
    t.select(t.v_varchar.concat(t.v_text)).compile().into()
}

pub fn xml_text() -> Query {
    t.select(t.v_xml.concat(t.v_text)).compile().into()
}
