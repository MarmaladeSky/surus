// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn bit_bit() -> Query {
    t.select(t.v_bit ^ t.v_bit).compile().into()
}

pub fn box_box() -> Query {
    t.select(t.v_box ^ t.v_box).compile().into()
}

pub fn int2_int2() -> Query {
    t.select(t.v_int2 ^ t.v_int2).compile().into()
}

pub fn int4_int4() -> Query {
    t.select(t.v_int4 ^ t.v_int4).compile().into()
}

pub fn int8_int8() -> Query {
    t.select(t.v_int8 ^ t.v_int8).compile().into()
}

pub fn line_line() -> Query {
    t.select(t.v_line ^ t.v_line).compile().into()
}

pub fn lseg_lseg() -> Query {
    t.select(t.v_lseg ^ t.v_lseg).compile().into()
}

pub fn path() -> Query {
    t.select(t.v_path.npoints()).compile().into()
}

pub fn polygon() -> Query {
    t.select(t.v_polygon.npoints()).compile().into()
}
