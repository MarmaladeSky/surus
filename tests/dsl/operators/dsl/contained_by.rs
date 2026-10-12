#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn box_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_box.contained_by(s.v_box))
        .into()
}

pub fn circle_circle() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_circle.contained_by(s.v_circle))
        .into()
}

pub fn int4_array_int4_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4_array.contained_by(s.v_int4_array))
        .into()
}

pub fn int4_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4.contained_by(s.v_int4multirange))
        .into()
}

pub fn int4_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4.contained_by(s.v_int4range))
        .into()
}

pub fn int4multirange_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4multirange.contained_by(s.v_int4multirange))
        .into()
}

pub fn int4multirange_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4multirange.contained_by(s.v_int4range))
        .into()
}

pub fn int4range_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4range.contained_by(s.v_int4multirange))
        .into()
}

pub fn int4range_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4range.contained_by(s.v_int4range))
        .into()
}

pub fn jsonb_jsonb() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_jsonb.contained_by(s.v_jsonb))
        .into()
}

pub fn lseg_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_lseg.contained_by(s.v_box))
        .into()
}

pub fn lseg_line() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_lseg.contained_by(s.v_line))
        .into()
}

pub fn point_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_point.contained_by(s.v_box))
        .into()
}

pub fn point_circle() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_point.contained_by(s.v_circle))
        .into()
}

pub fn point_line() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_point.contained_by(s.v_line))
        .into()
}

pub fn point_lseg() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_point.contained_by(s.v_lseg))
        .into()
}

pub fn point_path() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_point.contained_by(s.v_path))
        .into()
}

pub fn point_polygon() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_point.contained_by(s.v_polygon))
        .into()
}

pub fn polygon_polygon() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_polygon.contained_by(s.v_polygon))
        .into()
}

pub fn text_array_text_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_text_array.contained_by(s.v_text_array))
        .into()
}

pub fn timestamp_tsrange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_timestamp.contained_by(s.v_tsrange))
        .into()
}

pub fn tsquery_tsquery() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsquery.contained_by(s.v_tsquery))
        .into()
}

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsrange.contained_by(s.v_tsrange))
        .into()
}
