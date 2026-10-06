// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn bit_bit() -> Query {
    t.select(t.v_bit | t.v_bit).compile().into()
}

pub fn inet_inet() -> Query {
    t.select(t.v_inet | t.v_inet).compile().into()
}

pub fn int2_int2() -> Query {
    t.select(t.v_int2 | t.v_int2).compile().into()
}

pub fn int4_int4() -> Query {
    t.select(t.v_int4 | t.v_int4).compile().into()
}

pub fn int8_int8() -> Query {
    t.select(t.v_int8 | t.v_int8).compile().into()
}

pub fn macaddr8_macaddr8() -> Query {
    t.select(t.v_macaddr8 | t.v_macaddr8).compile().into()
}

pub fn macaddr_macaddr() -> Query {
    t.select(t.v_macaddr | t.v_macaddr).compile().into()
}
