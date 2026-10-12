#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn box_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_box.above(s.v_box)).into()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_point.above(s.v_point)).into()
}
