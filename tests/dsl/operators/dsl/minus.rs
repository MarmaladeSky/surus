use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn box_point() -> Query<(), (t::Nullable<t::PgBox>,)> {
    placeholder()
}

pub fn circle_point() -> Query<(), (t::Nullable<t::Circle>,)> {
    placeholder()
}

pub fn date_date() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn date_int4() -> Query<(), (t::Nullable<t::Date>,)> {
    placeholder()
}

pub fn date_interval() -> Query<(), (t::Nullable<t::Timestamp>,)> {
    placeholder()
}

pub fn float4() -> Query<(), (t::Nullable<t::Float4>,)> {
    placeholder()
}

pub fn float4_float4() -> Query<(), (t::Nullable<t::Float4>,)> {
    placeholder()
}

pub fn float4_float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}

pub fn float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}

pub fn float8_float4() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}

pub fn float8_float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}

pub fn inet_inet() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn inet_int8() -> Query<(), (t::Nullable<t::Inet>,)> {
    placeholder()
}

pub fn int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    placeholder()
}

pub fn int2_int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    placeholder()
}

pub fn int2_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn int2_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn int4_int2() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn int4_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn int4multirange_int4multirange() -> Query<(), (t::Nullable<t::Multirange<t::Int4>>,)> {
    placeholder()
}

pub fn int4range_int4range() -> Query<(), (t::Nullable<t::Range<t::Int4>>,)> {
    placeholder()
}

pub fn int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn int8_int2() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn int8_int4() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn interval() -> Query<(), (t::Nullable<t::Interval>,)> {
    placeholder()
}

pub fn interval_interval() -> Query<(), (t::Nullable<t::Interval>,)> {
    placeholder()
}

pub fn jsonb_int4() -> Query<(), (t::Nullable<t::Jsonb>,)> {
    placeholder()
}

pub fn jsonb_text() -> Query<(), (t::Nullable<t::Jsonb>,)> {
    placeholder()
}

pub fn jsonb_text_array() -> Query<(), (t::Nullable<t::Jsonb>,)> {
    placeholder()
}

pub fn money_money() -> Query<(), (t::Nullable<t::Money>,)> {
    placeholder()
}

pub fn numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    placeholder()
}

pub fn numeric_numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    placeholder()
}

pub fn path_point() -> Query<(), (t::Nullable<t::Path>,)> {
    placeholder()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn time_interval() -> Query<(), (t::Nullable<t::Time>,)> {
    placeholder()
}

pub fn time_time() -> Query<(), (t::Nullable<t::Interval>,)> {
    placeholder()
}

pub fn timestamp_interval() -> Query<(), (t::Nullable<t::Timestamp>,)> {
    placeholder()
}

pub fn timestamp_timestamp() -> Query<(), (t::Nullable<t::Interval>,)> {
    placeholder()
}

pub fn timestamptz_interval() -> Query<(), (t::Nullable<t::Timestamptz>,)> {
    placeholder()
}

pub fn timestamptz_timestamptz() -> Query<(), (t::Nullable<t::Interval>,)> {
    placeholder()
}

pub fn timetz_interval() -> Query<(), (t::Nullable<t::Timetz>,)> {
    placeholder()
}

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Range<t::Timestamp>>,)> {
    placeholder()
}
