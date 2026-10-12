#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn bit_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_bit.concat(s.v_text)).into()
}

pub fn bool_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_bool.concat(s.v_text)).into()
}

pub fn box_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_box.concat(s.v_text)).into()
}

pub fn bpchar_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_bpchar.concat(s.v_text)).into()
}

pub fn bytea_bytea() -> Query<(), (t::Nullable<t::Bytea>,)> {
    type_samples.select(|s| s.v_bytea.concat(s.v_bytea)).into()
}

pub fn bytea_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_bytea.concat(s.v_text)).into()
}

pub fn cidr_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_cidr.concat(s.v_text)).into()
}

pub fn circle_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_circle.concat(s.v_text)).into()
}

pub fn date_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_date.concat(s.v_text)).into()
}

pub fn float4_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_float4.concat(s.v_text)).into()
}

pub fn float8_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_float8.concat(s.v_text)).into()
}

pub fn inet_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_inet.concat(s.v_text)).into()
}

pub fn int2_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_int2.concat(s.v_text)).into()
}

pub fn int4_array_int4() -> Query<(), (t::Nullable<t::Array<t::Int4>>,)> {
    type_samples
        .select(|s| s.v_int4_array.concat(s.v_int4))
        .into()
}

pub fn int4_array_int4_array() -> Query<(), (t::Nullable<t::Array<t::Int4>>,)> {
    type_samples
        .select(|s| s.v_int4_array.concat(s.v_int4_array))
        .into()
}

pub fn int4_int4_array() -> Query<(), (t::Nullable<t::Array<t::Int4>>,)> {
    type_samples
        .select(|s| s.v_int4.concat(s.v_int4_array))
        .into()
}

pub fn int4_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_int4.concat(s.v_text)).into()
}

pub fn int4multirange_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_int4multirange.concat(s.v_text))
        .into()
}

pub fn int4range_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_int4range.concat(s.v_text))
        .into()
}

pub fn int8_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_int8.concat(s.v_text)).into()
}

pub fn interval_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_interval.concat(s.v_text))
        .into()
}

pub fn json_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_json.concat(s.v_text)).into()
}

pub fn jsonb_jsonb() -> Query<(), (t::Nullable<t::Jsonb>,)> {
    type_samples.select(|s| s.v_jsonb.concat(s.v_jsonb)).into()
}

pub fn jsonb_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_jsonb.concat(s.v_text)).into()
}

pub fn jsonpath_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_jsonpath.concat(s.v_text))
        .into()
}

pub fn line_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_line.concat(s.v_text)).into()
}

pub fn lseg_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_lseg.concat(s.v_text)).into()
}

pub fn macaddr8_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_macaddr8.concat(s.v_text))
        .into()
}

pub fn macaddr_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_macaddr.concat(s.v_text)).into()
}

pub fn money_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_money.concat(s.v_text)).into()
}

pub fn mood_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_mood.concat(s.v_text)).into()
}

pub fn numeric_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_numeric.concat(s.v_text)).into()
}

pub fn path_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_path.concat(s.v_text)).into()
}

pub fn point_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_point.concat(s.v_text)).into()
}

pub fn polygon_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_polygon.concat(s.v_text)).into()
}

pub fn price_tag_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_price_tag.concat(s.v_text))
        .into()
}

pub fn text_array_text() -> Query<(), (t::Nullable<t::Array<t::Text>>,)> {
    type_samples
        .select(|s| s.v_text_array.concat(s.v_text))
        .into()
}

pub fn text_array_text_array() -> Query<(), (t::Nullable<t::Array<t::Text>>,)> {
    type_samples
        .select(|s| s.v_text_array.concat(s.v_text_array))
        .into()
}

pub fn text_bit() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_bit)).into()
}

pub fn text_bool() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_bool)).into()
}

pub fn text_box() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_box)).into()
}

pub fn text_bpchar() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_bpchar)).into()
}

pub fn text_bytea() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_bytea)).into()
}

pub fn text_cidr() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_cidr)).into()
}

pub fn text_circle() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_circle)).into()
}

pub fn text_date() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_date)).into()
}

pub fn text_float4() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_float4)).into()
}

pub fn text_float8() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_float8)).into()
}

pub fn text_inet() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_inet)).into()
}

pub fn text_int2() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_int2)).into()
}

pub fn text_int4() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_int4)).into()
}

pub fn text_int4multirange() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_text.concat(s.v_int4multirange))
        .into()
}

pub fn text_int4range() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_text.concat(s.v_int4range))
        .into()
}

pub fn text_int8() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_int8)).into()
}

pub fn text_interval() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_text.concat(s.v_interval))
        .into()
}

pub fn text_json() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_json)).into()
}

pub fn text_jsonb() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_jsonb)).into()
}

pub fn text_jsonpath() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_text.concat(s.v_jsonpath))
        .into()
}

pub fn text_line() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_line)).into()
}

pub fn text_lseg() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_lseg)).into()
}

pub fn text_macaddr() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_macaddr)).into()
}

pub fn text_macaddr8() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_text.concat(s.v_macaddr8))
        .into()
}

pub fn text_money() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_money)).into()
}

pub fn text_mood() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_mood)).into()
}

pub fn text_numeric() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_numeric)).into()
}

pub fn text_path() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_path)).into()
}

pub fn text_point() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_point)).into()
}

pub fn text_polygon() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_polygon)).into()
}

pub fn text_price_tag() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_text.concat(s.v_price_tag))
        .into()
}

pub fn text_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_text)).into()
}

pub fn text_text_array() -> Query<(), (t::Nullable<t::Array<t::Text>>,)> {
    type_samples
        .select(|s| s.v_text.concat(s.v_text_array))
        .into()
}

pub fn text_time() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_time)).into()
}

pub fn text_timestamp() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_text.concat(s.v_timestamp))
        .into()
}

pub fn text_timestamptz() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_text.concat(s.v_timestamptz))
        .into()
}

pub fn text_timetz() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_timetz)).into()
}

pub fn text_tsquery() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_tsquery)).into()
}

pub fn text_tsrange() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_tsrange)).into()
}

pub fn text_tsvector() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_text.concat(s.v_tsvector))
        .into()
}

pub fn text_uuid() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_uuid)).into()
}

pub fn text_varbit() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_varbit)).into()
}

pub fn text_varchar() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_varchar)).into()
}

pub fn text_xml() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_text.concat(s.v_xml)).into()
}

pub fn time_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_time.concat(s.v_text)).into()
}

pub fn timestamp_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_timestamp.concat(s.v_text))
        .into()
}

pub fn timestamptz_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_timestamptz.concat(s.v_text))
        .into()
}

pub fn timetz_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_timetz.concat(s.v_text)).into()
}

pub fn tsquery_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_tsquery.concat(s.v_text)).into()
}

pub fn tsquery_tsquery() -> Query<(), (t::Nullable<t::Tsquery>,)> {
    type_samples
        .select(|s| s.v_tsquery.ts_or(s.v_tsquery))
        .into()
}

pub fn tsrange_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_tsrange.concat(s.v_text)).into()
}

pub fn tsvector_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_tsvector.concat(s.v_text))
        .into()
}

pub fn tsvector_tsvector() -> Query<(), (t::Nullable<t::Tsvector>,)> {
    type_samples
        .select(|s| s.v_tsvector.concat(s.v_tsvector))
        .into()
}

pub fn uuid_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_uuid.concat(s.v_text)).into()
}

pub fn varbit_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_varbit.concat(s.v_text)).into()
}

pub fn varbit_varbit() -> Query<(), (t::Nullable<t::Varbit>,)> {
    type_samples
        .select(|s| s.v_varbit.concat(s.v_varbit))
        .into()
}

pub fn varchar_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_varchar.concat(s.v_text)).into()
}

pub fn xml_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_xml.concat(s.v_text)).into()
}
