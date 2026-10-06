// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn int2_int2() -> Query {
    t.select(t.v_int2 % t.v_int2).compile().into()
}

pub fn int4_int4() -> Query {
    t.select(t.v_int4 % t.v_int4).compile().into()
}

pub fn int8_int8() -> Query {
    t.select(t.v_int8 % t.v_int8).compile().into()
}

pub fn numeric_numeric() -> Query {
    t.select(t.v_numeric % t.v_numeric).compile().into()
}
