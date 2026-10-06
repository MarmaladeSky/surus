// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn bit() -> Query {
    t.select(!t.v_bit).compile().into()
}

pub fn bpchar_text() -> Query {
    t.select(t.v_bpchar.regex(t.v_text)).compile().into()
}

pub fn inet() -> Query {
    t.select(!t.v_inet).compile().into()
}

pub fn int2() -> Query {
    t.select(!t.v_int2).compile().into()
}

pub fn int4() -> Query {
    t.select(!t.v_int4).compile().into()
}

pub fn int8() -> Query {
    t.select(!t.v_int8).compile().into()
}

pub fn macaddr() -> Query {
    t.select(!t.v_macaddr).compile().into()
}

pub fn macaddr8() -> Query {
    t.select(!t.v_macaddr8).compile().into()
}

pub fn text_text() -> Query {
    t.select(t.v_text.regex(t.v_text)).compile().into()
}
