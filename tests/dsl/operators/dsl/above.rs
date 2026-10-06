// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn box_box() -> Query {
    t.select(t.v_box.above(t.v_box)).compile().into()
}

pub fn point_point() -> Query {
    t.select(t.v_point.above(t.v_point)).compile().into()
}
