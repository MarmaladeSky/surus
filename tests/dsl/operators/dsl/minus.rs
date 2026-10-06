// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn box_point() -> Query {
    t.select(t.v_box - t.v_point).compile().into()
}

pub fn circle_point() -> Query {
    t.select(t.v_circle - t.v_point).compile().into()
}

pub fn date_date() -> Query {
    t.select(t.v_date - t.v_date).compile().into()
}

pub fn date_int4() -> Query {
    t.select(t.v_date - t.v_int4).compile().into()
}

pub fn date_interval() -> Query {
    t.select(t.v_date - t.v_interval).compile().into()
}

pub fn float4() -> Query {
    t.select(-t.v_float4).compile().into()
}

pub fn float4_float4() -> Query {
    t.select(t.v_float4 - t.v_float4).compile().into()
}

pub fn float4_float8() -> Query {
    t.select(t.v_float4 - t.v_float8).compile().into()
}

pub fn float8() -> Query {
    t.select(-t.v_float8).compile().into()
}

pub fn float8_float4() -> Query {
    t.select(t.v_float8 - t.v_float4).compile().into()
}

pub fn float8_float8() -> Query {
    t.select(t.v_float8 - t.v_float8).compile().into()
}

pub fn inet_inet() -> Query {
    t.select(t.v_inet - t.v_inet).compile().into()
}

pub fn inet_int8() -> Query {
    t.select(t.v_inet - t.v_int8).compile().into()
}

pub fn int2() -> Query {
    t.select(-t.v_int2).compile().into()
}

pub fn int2_int2() -> Query {
    t.select(t.v_int2 - t.v_int2).compile().into()
}

pub fn int2_int4() -> Query {
    t.select(t.v_int2 - t.v_int4).compile().into()
}

pub fn int2_int8() -> Query {
    t.select(t.v_int2 - t.v_int8).compile().into()
}

pub fn int4() -> Query {
    t.select(-t.v_int4).compile().into()
}

pub fn int4_int2() -> Query {
    t.select(t.v_int4 - t.v_int2).compile().into()
}

pub fn int4_int4() -> Query {
    t.select(t.v_int4 - t.v_int4).compile().into()
}

pub fn int4_int8() -> Query {
    t.select(t.v_int4 - t.v_int8).compile().into()
}

pub fn int4multirange_int4multirange() -> Query {
    t.select(t.v_int4multirange - t.v_int4multirange)
        .compile()
        .into()
}

pub fn int4range_int4range() -> Query {
    t.select(t.v_int4range - t.v_int4range).compile().into()
}

pub fn int8() -> Query {
    t.select(-t.v_int8).compile().into()
}

pub fn int8_int2() -> Query {
    t.select(t.v_int8 - t.v_int2).compile().into()
}

pub fn int8_int4() -> Query {
    t.select(t.v_int8 - t.v_int4).compile().into()
}

pub fn int8_int8() -> Query {
    t.select(t.v_int8 - t.v_int8).compile().into()
}

pub fn interval() -> Query {
    t.select(-t.v_interval).compile().into()
}

pub fn interval_interval() -> Query {
    t.select(t.v_interval - t.v_interval).compile().into()
}

pub fn jsonb_int4() -> Query {
    t.select(t.v_jsonb - t.v_int4).compile().into()
}

pub fn jsonb_text() -> Query {
    t.select(t.v_jsonb - t.v_text).compile().into()
}

pub fn jsonb_text_array() -> Query {
    t.select(t.v_jsonb - t.v_text_array).compile().into()
}

pub fn money_money() -> Query {
    t.select(t.v_money - t.v_money).compile().into()
}

pub fn numeric() -> Query {
    t.select(-t.v_numeric).compile().into()
}

pub fn numeric_numeric() -> Query {
    t.select(t.v_numeric - t.v_numeric).compile().into()
}

pub fn path_point() -> Query {
    t.select(t.v_path - t.v_point).compile().into()
}

pub fn point_point() -> Query {
    t.select(t.v_point - t.v_point).compile().into()
}

pub fn time_interval() -> Query {
    t.select(t.v_time - t.v_interval).compile().into()
}

pub fn time_time() -> Query {
    t.select(t.v_time - t.v_time).compile().into()
}

pub fn timestamp_interval() -> Query {
    t.select(t.v_timestamp - t.v_interval).compile().into()
}

pub fn timestamp_timestamp() -> Query {
    t.select(t.v_timestamp - t.v_timestamp).compile().into()
}

pub fn timestamptz_interval() -> Query {
    t.select(t.v_timestamptz - t.v_interval).compile().into()
}

pub fn timestamptz_timestamptz() -> Query {
    t.select(t.v_timestamptz - t.v_timestamptz).compile().into()
}

pub fn timetz_interval() -> Query {
    t.select(t.v_timetz - t.v_interval).compile().into()
}

pub fn tsrange_tsrange() -> Query {
    t.select(t.v_tsrange - t.v_tsrange).compile().into()
}
