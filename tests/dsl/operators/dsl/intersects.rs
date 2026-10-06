// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn box_box() -> Query {
    t.select(t.v_box.intersects(t.v_box)).compile().into()
}

pub fn line_box() -> Query {
    t.select(t.v_line.intersects(t.v_box)).compile().into()
}

pub fn line_line() -> Query {
    t.select(t.v_line.intersects(t.v_line)).compile().into()
}

pub fn lseg_box() -> Query {
    t.select(t.v_lseg.intersects(t.v_box)).compile().into()
}

pub fn lseg_line() -> Query {
    t.select(t.v_lseg.intersects(t.v_line)).compile().into()
}

pub fn lseg_lseg() -> Query {
    t.select(t.v_lseg.intersects(t.v_lseg)).compile().into()
}

pub fn path_path() -> Query {
    t.select(t.v_path.intersects(t.v_path)).compile().into()
}
