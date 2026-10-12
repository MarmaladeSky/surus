#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn box_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_box.intersects(s.v_box)).into()
}

pub fn line_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_line.intersects(s.v_box)).into()
}

pub fn line_line() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_line.intersects(s.v_line))
        .into()
}

pub fn lseg_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_lseg.intersects(s.v_box)).into()
}

pub fn lseg_line() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_lseg.intersects(s.v_line))
        .into()
}

pub fn lseg_lseg() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_lseg.intersects(s.v_lseg))
        .into()
}

pub fn path_path() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_path.intersects(s.v_path))
        .into()
}
