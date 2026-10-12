#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn box_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_box.overabove(s.v_box)).into()
}

pub fn circle_circle() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_circle.overabove(s.v_circle))
        .into()
}

pub fn polygon_polygon() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_polygon.overabove(s.v_polygon))
        .into()
}
