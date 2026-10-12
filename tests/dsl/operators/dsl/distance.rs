#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn box_box() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_box.distance(s.v_box)).into()
}

pub fn box_lseg() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_box.distance(s.v_lseg)).into()
}

pub fn box_point() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_box.distance(s.v_point)).into()
}

pub fn circle_circle() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples
        .select(|s| s.v_circle.distance(s.v_circle))
        .into()
}

pub fn circle_point() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples
        .select(|s| s.v_circle.distance(s.v_point))
        .into()
}

pub fn circle_polygon() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples
        .select(|s| s.v_circle.distance(s.v_polygon))
        .into()
}

pub fn line_line() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_line.distance(s.v_line)).into()
}

pub fn line_lseg() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_line.distance(s.v_lseg)).into()
}

pub fn line_point() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_line.distance(s.v_point)).into()
}

pub fn lseg_box() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_lseg.distance(s.v_box)).into()
}

pub fn lseg_line() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_lseg.distance(s.v_line)).into()
}

pub fn lseg_lseg() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_lseg.distance(s.v_lseg)).into()
}

pub fn lseg_point() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_lseg.distance(s.v_point)).into()
}

pub fn path_path() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_path.distance(s.v_path)).into()
}

pub fn path_point() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_path.distance(s.v_point)).into()
}

pub fn point_box() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_point.distance(s.v_box)).into()
}

pub fn point_circle() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples
        .select(|s| s.v_point.distance(s.v_circle))
        .into()
}

pub fn point_line() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_point.distance(s.v_line)).into()
}

pub fn point_lseg() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_point.distance(s.v_lseg)).into()
}

pub fn point_path() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_point.distance(s.v_path)).into()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples
        .select(|s| s.v_point.distance(s.v_point))
        .into()
}

pub fn point_polygon() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples
        .select(|s| s.v_point.distance(s.v_polygon))
        .into()
}

pub fn polygon_circle() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples
        .select(|s| s.v_polygon.distance(s.v_circle))
        .into()
}

pub fn polygon_point() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples
        .select(|s| s.v_polygon.distance(s.v_point))
        .into()
}

pub fn polygon_polygon() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples
        .select(|s| s.v_polygon.distance(s.v_polygon))
        .into()
}

pub fn tsquery_tsquery() -> Query<(), (t::Nullable<t::Tsquery>,)> {
    type_samples
        .select(|s| s.v_tsquery.followed_by(s.v_tsquery))
        .into()
}
