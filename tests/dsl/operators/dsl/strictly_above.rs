// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn box_box() -> Query {
    t.select(t.v_box.strictly_above(t.v_box)).compile().into()
}

pub fn circle_circle() -> Query {
    t.select(t.v_circle.strictly_above(t.v_circle))
        .compile()
        .into()
}

pub fn point_point() -> Query {
    t.select(t.v_point.strictly_above(t.v_point))
        .compile()
        .into()
}

pub fn polygon_polygon() -> Query {
    t.select(t.v_polygon.strictly_above(t.v_polygon))
        .compile()
        .into()
}
