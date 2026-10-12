#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn box_point() -> Query<(), (t::Nullable<t::PgBox>,)> {
    type_samples.select(|s| s.v_box - s.v_point).into()
}

pub fn circle_point() -> Query<(), (t::Nullable<t::Circle>,)> {
    type_samples.select(|s| s.v_circle - s.v_point).into()
}

pub fn date_date() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_date - s.v_date).into()
}

pub fn date_int4() -> Query<(), (t::Nullable<t::Date>,)> {
    type_samples.select(|s| s.v_date - s.v_int4).into()
}

pub fn date_interval() -> Query<(), (t::Nullable<t::Timestamp>,)> {
    type_samples.select(|s| s.v_date - s.v_interval).into()
}

pub fn float4() -> Query<(), (t::Nullable<t::Float4>,)> {
    type_samples.select(|s| -s.v_float4).into()
}

pub fn float4_float4() -> Query<(), (t::Nullable<t::Float4>,)> {
    type_samples.select(|s| s.v_float4 - s.v_float4).into()
}

pub fn float4_float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_float4 - s.v_float8).into()
}

pub fn float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| -s.v_float8).into()
}

pub fn float8_float4() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_float8 - s.v_float4).into()
}

pub fn float8_float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_float8 - s.v_float8).into()
}

pub fn inet_inet() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_inet - s.v_inet).into()
}

pub fn inet_int8() -> Query<(), (t::Nullable<t::Inet>,)> {
    type_samples.select(|s| s.v_inet - s.v_int8).into()
}

pub fn int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    type_samples.select(|s| -s.v_int2).into()
}

pub fn int2_int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    type_samples.select(|s| s.v_int2 - s.v_int2).into()
}

pub fn int2_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int2 - s.v_int4).into()
}

pub fn int2_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int2 - s.v_int8).into()
}

pub fn int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| -s.v_int4).into()
}

pub fn int4_int2() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int4 - s.v_int2).into()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int4 - s.v_int4).into()
}

pub fn int4_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int4 - s.v_int8).into()
}

pub fn int4multirange_int4multirange() -> Query<(), (t::Nullable<t::Multirange<t::Int4>>,)> {
    type_samples
        .select(|s| s.v_int4multirange.difference(s.v_int4multirange))
        .into()
}

pub fn int4range_int4range() -> Query<(), (t::Nullable<t::Range<t::Int4>>,)> {
    type_samples
        .select(|s| s.v_int4range.difference(s.v_int4range))
        .into()
}

pub fn int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| -s.v_int8).into()
}

pub fn int8_int2() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8 - s.v_int2).into()
}

pub fn int8_int4() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8 - s.v_int4).into()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8 - s.v_int8).into()
}

pub fn interval() -> Query<(), (t::Nullable<t::Interval>,)> {
    type_samples.select(|s| -s.v_interval).into()
}

pub fn interval_interval() -> Query<(), (t::Nullable<t::Interval>,)> {
    type_samples.select(|s| s.v_interval - s.v_interval).into()
}

pub fn jsonb_int4() -> Query<(), (t::Nullable<t::Jsonb>,)> {
    type_samples.select(|s| s.v_jsonb.remove(s.v_int4)).into()
}

pub fn jsonb_text() -> Query<(), (t::Nullable<t::Jsonb>,)> {
    type_samples.select(|s| s.v_jsonb.remove(s.v_text)).into()
}

pub fn jsonb_text_array() -> Query<(), (t::Nullable<t::Jsonb>,)> {
    type_samples
        .select(|s| s.v_jsonb.remove(s.v_text_array))
        .into()
}

pub fn money_money() -> Query<(), (t::Nullable<t::Money>,)> {
    type_samples.select(|s| s.v_money - s.v_money).into()
}

pub fn numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    type_samples.select(|s| -s.v_numeric).into()
}

pub fn numeric_numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    type_samples.select(|s| s.v_numeric - s.v_numeric).into()
}

pub fn path_point() -> Query<(), (t::Nullable<t::Path>,)> {
    type_samples.select(|s| s.v_path - s.v_point).into()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples.select(|s| s.v_point - s.v_point).into()
}

pub fn time_interval() -> Query<(), (t::Nullable<t::Time>,)> {
    type_samples.select(|s| s.v_time - s.v_interval).into()
}

pub fn time_time() -> Query<(), (t::Nullable<t::Interval>,)> {
    type_samples.select(|s| s.v_time - s.v_time).into()
}

pub fn timestamp_interval() -> Query<(), (t::Nullable<t::Timestamp>,)> {
    type_samples.select(|s| s.v_timestamp - s.v_interval).into()
}

pub fn timestamp_timestamp() -> Query<(), (t::Nullable<t::Interval>,)> {
    type_samples
        .select(|s| s.v_timestamp - s.v_timestamp)
        .into()
}

pub fn timestamptz_interval() -> Query<(), (t::Nullable<t::Timestamptz>,)> {
    type_samples
        .select(|s| s.v_timestamptz - s.v_interval)
        .into()
}

pub fn timestamptz_timestamptz() -> Query<(), (t::Nullable<t::Interval>,)> {
    type_samples
        .select(|s| s.v_timestamptz - s.v_timestamptz)
        .into()
}

pub fn timetz_interval() -> Query<(), (t::Nullable<t::Timetz>,)> {
    type_samples.select(|s| s.v_timetz - s.v_interval).into()
}

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Range<t::Timestamp>>,)> {
    type_samples
        .select(|s| s.v_tsrange.difference(s.v_tsrange))
        .into()
}
