#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn line_lseg() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples
        .select(|s| s.v_line.closest_point(s.v_lseg))
        .into()
}

pub fn lseg_box() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples
        .select(|s| s.v_lseg.closest_point(s.v_box))
        .into()
}

pub fn lseg_lseg() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples
        .select(|s| s.v_lseg.closest_point(s.v_lseg))
        .into()
}

pub fn point_box() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples
        .select(|s| s.v_point.closest_point(s.v_box))
        .into()
}

pub fn point_line() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples
        .select(|s| s.v_point.closest_point(s.v_line))
        .into()
}

pub fn point_lseg() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples
        .select(|s| s.v_point.closest_point(s.v_lseg))
        .into()
}
