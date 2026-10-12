#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn bit_bit() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_bit.ne(s.v_bit)).into()
}

pub fn bool_bool() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_bool.ne(s.v_bool)).into()
}

pub fn bpchar_bpchar() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_bpchar.ne(s.v_bpchar)).into()
}

pub fn bytea_bytea() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_bytea.ne(s.v_bytea)).into()
}

pub fn char_char() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_char.ne(s.v_char)).into()
}

pub fn circle_circle() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_circle.ne(s.v_circle)).into()
}

pub fn date_date() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_date.ne(s.v_date)).into()
}

pub fn date_timestamp() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_date.ne(s.v_timestamp)).into()
}

pub fn date_timestamptz() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_date.ne(s.v_timestamptz)).into()
}

pub fn float4_float4() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_float4.ne(s.v_float4)).into()
}

pub fn float4_float8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_float4.ne(s.v_float8)).into()
}

pub fn float8_float4() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_float8.ne(s.v_float4)).into()
}

pub fn float8_float8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_float8.ne(s.v_float8)).into()
}

pub fn inet_inet() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_inet.ne(s.v_inet)).into()
}

pub fn int2_int2() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int2.ne(s.v_int2)).into()
}

pub fn int2_int4() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int2.ne(s.v_int4)).into()
}

pub fn int2_int8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int2.ne(s.v_int8)).into()
}

pub fn int4_array_int4_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4_array.ne(s.v_int4_array))
        .into()
}

pub fn int4_int2() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int4.ne(s.v_int2)).into()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int4.ne(s.v_int4)).into()
}

pub fn int4_int8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int4.ne(s.v_int8)).into()
}

pub fn int4multirange_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4multirange.ne(s.v_int4multirange))
        .into()
}

pub fn int4range_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4range.ne(s.v_int4range))
        .into()
}

pub fn int8_int2() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int8.ne(s.v_int2)).into()
}

pub fn int8_int4() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int8.ne(s.v_int4)).into()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_int8.ne(s.v_int8)).into()
}

pub fn interval_interval() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_interval.ne(s.v_interval))
        .into()
}

pub fn jsonb_jsonb() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_jsonb.ne(s.v_jsonb)).into()
}

pub fn lseg_lseg() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_lseg.ne(s.v_lseg)).into()
}

pub fn macaddr8_macaddr8() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_macaddr8.ne(s.v_macaddr8))
        .into()
}

pub fn macaddr_macaddr() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_macaddr.ne(s.v_macaddr)).into()
}

pub fn money_money() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_money.ne(s.v_money)).into()
}

pub fn mood_mood() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_mood.ne(s.v_mood)).into()
}

pub fn numeric_numeric() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_numeric.ne(s.v_numeric)).into()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_point.ne(s.v_point)).into()
}

pub fn price_tag_price_tag() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_price_tag.ne(s.v_price_tag))
        .into()
}

pub fn text_array_text_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_text_array.ne(s.v_text_array))
        .into()
}

pub fn text_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_text.ne(s.v_text)).into()
}

pub fn time_time() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_time.ne(s.v_time)).into()
}

pub fn timestamp_date() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_timestamp.ne(s.v_date)).into()
}

pub fn timestamp_timestamp() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_timestamp.ne(s.v_timestamp))
        .into()
}

pub fn timestamp_timestamptz() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_timestamp.ne(s.v_timestamptz))
        .into()
}

pub fn timestamptz_date() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_timestamptz.ne(s.v_date)).into()
}

pub fn timestamptz_timestamp() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_timestamptz.ne(s.v_timestamp))
        .into()
}

pub fn timestamptz_timestamptz() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_timestamptz.ne(s.v_timestamptz))
        .into()
}

pub fn timetz_timetz() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_timetz.ne(s.v_timetz)).into()
}

pub fn tsquery_tsquery() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_tsquery.ne(s.v_tsquery)).into()
}

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_tsrange.ne(s.v_tsrange)).into()
}

pub fn tsvector_tsvector() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsvector.ne(s.v_tsvector))
        .into()
}

pub fn uuid_uuid() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_uuid.ne(s.v_uuid)).into()
}

pub fn varbit_varbit() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_varbit.ne(s.v_varbit)).into()
}
