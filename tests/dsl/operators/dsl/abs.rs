// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn float4() -> Query {
    t.select(t.v_float4.abs()).compile().into()
}

pub fn float8() -> Query {
    t.select(t.v_float8.abs()).compile().into()
}

pub fn int2() -> Query {
    t.select(t.v_int2.abs()).compile().into()
}

pub fn int4() -> Query {
    t.select(t.v_int4.abs()).compile().into()
}

pub fn int8() -> Query {
    t.select(t.v_int8.abs()).compile().into()
}

pub fn numeric() -> Query {
    t.select(t.v_numeric.abs()).compile().into()
}
