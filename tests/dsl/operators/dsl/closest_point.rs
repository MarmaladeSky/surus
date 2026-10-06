// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn line_lseg() -> Query {
    t.select(t.v_line.closest_point(t.v_lseg)).compile().into()
}

pub fn lseg_box() -> Query {
    t.select(t.v_lseg.closest_point(t.v_box)).compile().into()
}

pub fn lseg_lseg() -> Query {
    t.select(t.v_lseg.closest_point(t.v_lseg)).compile().into()
}

pub fn point_box() -> Query {
    t.select(t.v_point.closest_point(t.v_box)).compile().into()
}

pub fn point_line() -> Query {
    t.select(t.v_point.closest_point(t.v_line)).compile().into()
}

pub fn point_lseg() -> Query {
    t.select(t.v_point.closest_point(t.v_lseg)).compile().into()
}
