// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn box_box() -> Query {
    t.select(t.v_box.distance(t.v_box)).compile().into()
}

pub fn box_lseg() -> Query {
    t.select(t.v_box.distance(t.v_lseg)).compile().into()
}

pub fn box_point() -> Query {
    t.select(t.v_box.distance(t.v_point)).compile().into()
}

pub fn circle_circle() -> Query {
    t.select(t.v_circle.distance(t.v_circle)).compile().into()
}

pub fn circle_point() -> Query {
    t.select(t.v_circle.distance(t.v_point)).compile().into()
}

pub fn circle_polygon() -> Query {
    t.select(t.v_circle.distance(t.v_polygon)).compile().into()
}

pub fn line_line() -> Query {
    t.select(t.v_line.distance(t.v_line)).compile().into()
}

pub fn line_lseg() -> Query {
    t.select(t.v_line.distance(t.v_lseg)).compile().into()
}

pub fn line_point() -> Query {
    t.select(t.v_line.distance(t.v_point)).compile().into()
}

pub fn lseg_box() -> Query {
    t.select(t.v_lseg.distance(t.v_box)).compile().into()
}

pub fn lseg_line() -> Query {
    t.select(t.v_lseg.distance(t.v_line)).compile().into()
}

pub fn lseg_lseg() -> Query {
    t.select(t.v_lseg.distance(t.v_lseg)).compile().into()
}

pub fn lseg_point() -> Query {
    t.select(t.v_lseg.distance(t.v_point)).compile().into()
}

pub fn path_path() -> Query {
    t.select(t.v_path.distance(t.v_path)).compile().into()
}

pub fn path_point() -> Query {
    t.select(t.v_path.distance(t.v_point)).compile().into()
}

pub fn point_box() -> Query {
    t.select(t.v_point.distance(t.v_box)).compile().into()
}

pub fn point_circle() -> Query {
    t.select(t.v_point.distance(t.v_circle)).compile().into()
}

pub fn point_line() -> Query {
    t.select(t.v_point.distance(t.v_line)).compile().into()
}

pub fn point_lseg() -> Query {
    t.select(t.v_point.distance(t.v_lseg)).compile().into()
}

pub fn point_path() -> Query {
    t.select(t.v_point.distance(t.v_path)).compile().into()
}

pub fn point_point() -> Query {
    t.select(t.v_point.distance(t.v_point)).compile().into()
}

pub fn point_polygon() -> Query {
    t.select(t.v_point.distance(t.v_polygon)).compile().into()
}

pub fn polygon_circle() -> Query {
    t.select(t.v_polygon.distance(t.v_circle)).compile().into()
}

pub fn polygon_point() -> Query {
    t.select(t.v_polygon.distance(t.v_point)).compile().into()
}

pub fn polygon_polygon() -> Query {
    t.select(t.v_polygon.distance(t.v_polygon)).compile().into()
}

pub fn tsquery_tsquery() -> Query {
    t.select(t.v_tsquery.distance(t.v_tsquery)).compile().into()
}
