// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn box_point() -> Query {
    t.select(t.v_box * t.v_point).compile().into()
}

pub fn circle_point() -> Query {
    t.select(t.v_circle * t.v_point).compile().into()
}

pub fn float4_float4() -> Query {
    t.select(t.v_float4 * t.v_float4).compile().into()
}

pub fn float4_float8() -> Query {
    t.select(t.v_float4 * t.v_float8).compile().into()
}

pub fn float4_money() -> Query {
    t.select(t.v_float4 * t.v_money).compile().into()
}

pub fn float8_float4() -> Query {
    t.select(t.v_float8 * t.v_float4).compile().into()
}

pub fn float8_float8() -> Query {
    t.select(t.v_float8 * t.v_float8).compile().into()
}

pub fn float8_interval() -> Query {
    t.select(t.v_float8 * t.v_interval).compile().into()
}

pub fn float8_money() -> Query {
    t.select(t.v_float8 * t.v_money).compile().into()
}

pub fn int2_int2() -> Query {
    t.select(t.v_int2 * t.v_int2).compile().into()
}

pub fn int2_int4() -> Query {
    t.select(t.v_int2 * t.v_int4).compile().into()
}

pub fn int2_int8() -> Query {
    t.select(t.v_int2 * t.v_int8).compile().into()
}

pub fn int2_money() -> Query {
    t.select(t.v_int2 * t.v_money).compile().into()
}

pub fn int4_int2() -> Query {
    t.select(t.v_int4 * t.v_int2).compile().into()
}

pub fn int4_int4() -> Query {
    t.select(t.v_int4 * t.v_int4).compile().into()
}

pub fn int4_int8() -> Query {
    t.select(t.v_int4 * t.v_int8).compile().into()
}

pub fn int4_money() -> Query {
    t.select(t.v_int4 * t.v_money).compile().into()
}

pub fn int4multirange_int4multirange() -> Query {
    t.select(t.v_int4multirange * t.v_int4multirange)
        .compile()
        .into()
}

pub fn int4range_int4range() -> Query {
    t.select(t.v_int4range * t.v_int4range).compile().into()
}

pub fn int8_int2() -> Query {
    t.select(t.v_int8 * t.v_int2).compile().into()
}

pub fn int8_int4() -> Query {
    t.select(t.v_int8 * t.v_int4).compile().into()
}

pub fn int8_int8() -> Query {
    t.select(t.v_int8 * t.v_int8).compile().into()
}

pub fn int8_money() -> Query {
    t.select(t.v_int8 * t.v_money).compile().into()
}

pub fn interval_float8() -> Query {
    t.select(t.v_interval * t.v_float8).compile().into()
}

pub fn money_float4() -> Query {
    t.select(t.v_money * t.v_float4).compile().into()
}

pub fn money_float8() -> Query {
    t.select(t.v_money * t.v_float8).compile().into()
}

pub fn money_int2() -> Query {
    t.select(t.v_money * t.v_int2).compile().into()
}

pub fn money_int4() -> Query {
    t.select(t.v_money * t.v_int4).compile().into()
}

pub fn money_int8() -> Query {
    t.select(t.v_money * t.v_int8).compile().into()
}

pub fn numeric_numeric() -> Query {
    t.select(t.v_numeric * t.v_numeric).compile().into()
}

pub fn path_point() -> Query {
    t.select(t.v_path * t.v_point).compile().into()
}

pub fn point_point() -> Query {
    t.select(t.v_point * t.v_point).compile().into()
}

pub fn tsrange_tsrange() -> Query {
    t.select(t.v_tsrange * t.v_tsrange).compile().into()
}
