#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn bit_bit() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_bit.gt(s.v_bit)).into()
}

pub fn bool_bool() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_bool.gt(s.v_bool)).into()
}

pub fn box_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_box.gt(s.v_box)).into()
}

pub fn bpchar_bpchar() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_bpchar.gt(s.v_bpchar)).into()
}

pub fn bytea_bytea() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_bytea.gt(s.v_bytea)).into()
}

pub fn char_char() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_char.gt(s.v_char)).into()
}

pub fn circle_circle() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_circle.gt(s.v_circle)).into()
}

pub fn date_date() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_date.gt(s.v_date)).into()
}

pub fn date_timestamp() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_date.gt(s.v_timestamp)).into()
}

pub fn date_timestamptz() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_date.gt(s.v_timestamptz)).into()
}

pub fn float4_float4() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_float4.gt(s.v_float4)).into()
}

pub fn float4_float8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_float4.gt(s.v_float8)).into()
}

pub fn float8_float4() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_float8.gt(s.v_float4)).into()
}

pub fn float8_float8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_float8.gt(s.v_float8)).into()
}

pub fn inet_inet() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_inet.gt(s.v_inet)).into()
}

pub fn int2_int2() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int2.gt(s.v_int2)).into()
}

pub fn int2_int4() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int2.gt(s.v_int4)).into()
}

pub fn int2_int8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int2.gt(s.v_int8)).into()
}

pub fn int4_array_int4_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4_array.gt(s.v_int4_array))
        .into()
}

pub fn int4_int2() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int4.gt(s.v_int2)).into()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int4.gt(s.v_int4)).into()
}

pub fn int4_int8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int4.gt(s.v_int8)).into()
}

pub fn int4multirange_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4multirange.gt(s.v_int4multirange))
        .into()
}

pub fn int4range_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4range.gt(s.v_int4range))
        .into()
}

pub fn int8_int2() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int8.gt(s.v_int2)).into()
}

pub fn int8_int4() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int8.gt(s.v_int4)).into()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int8.gt(s.v_int8)).into()
}

pub fn interval_interval() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_interval.gt(s.v_interval))
        .into()
}

pub fn jsonb_jsonb() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_jsonb.gt(s.v_jsonb)).into()
}

pub fn lseg_lseg() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_lseg.gt(s.v_lseg)).into()
}

pub fn macaddr8_macaddr8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_macaddr8.gt(s.v_macaddr8))
        .into()
}

pub fn macaddr_macaddr() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_macaddr.gt(s.v_macaddr)).into()
}

pub fn money_money() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_money.gt(s.v_money)).into()
}

pub fn mood_mood() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_mood.gt(s.v_mood)).into()
}

pub fn numeric_numeric() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_numeric.gt(s.v_numeric)).into()
}

pub fn path_path() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_path.gt(s.v_path)).into()
}

pub fn price_tag_price_tag() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_price_tag.gt(s.v_price_tag))
        .into()
}

pub fn text_array_text_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_text_array.gt(s.v_text_array))
        .into()
}

pub fn text_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_text.gt(s.v_text)).into()
}

pub fn time_time() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_time.gt(s.v_time)).into()
}

pub fn timestamp_date() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_timestamp.gt(s.v_date)).into()
}

pub fn timestamp_timestamp() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_timestamp.gt(s.v_timestamp))
        .into()
}

pub fn timestamp_timestamptz() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_timestamp.gt(s.v_timestamptz))
        .into()
}

pub fn timestamptz_date() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_timestamptz.gt(s.v_date)).into()
}

pub fn timestamptz_timestamp() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_timestamptz.gt(s.v_timestamp))
        .into()
}

pub fn timestamptz_timestamptz() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_timestamptz.gt(s.v_timestamptz))
        .into()
}

pub fn timetz_timetz() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_timetz.gt(s.v_timetz)).into()
}

pub fn tsquery_tsquery() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_tsquery.gt(s.v_tsquery)).into()
}

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_tsrange.gt(s.v_tsrange)).into()
}

pub fn tsvector_tsvector() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsvector.gt(s.v_tsvector))
        .into()
}

pub fn uuid_uuid() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_uuid.gt(s.v_uuid)).into()
}

pub fn varbit_varbit() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_varbit.gt(s.v_varbit)).into()
}
