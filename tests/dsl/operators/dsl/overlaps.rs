#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn box_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_box.overlaps(s.v_box)).into()
}

pub fn circle_circle() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_circle.overlaps(s.v_circle))
        .into()
}

pub fn inet_inet() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_inet.overlaps(s.v_inet)).into()
}

pub fn int4_array_int4_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4_array.overlaps(s.v_int4_array))
        .into()
}

pub fn int4multirange_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4multirange.overlaps(s.v_int4multirange))
        .into()
}

pub fn int4multirange_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4multirange.overlaps(s.v_int4range))
        .into()
}

pub fn int4range_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4range.overlaps(s.v_int4multirange))
        .into()
}

pub fn int4range_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4range.overlaps(s.v_int4range))
        .into()
}

pub fn polygon_polygon() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_polygon.overlaps(s.v_polygon))
        .into()
}

pub fn text_array_text_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_text_array.overlaps(s.v_text_array))
        .into()
}

pub fn tsquery_tsquery() -> Query<(), (t::Nullable<t::Tsquery>,)> {
    type_samples
        .select(|s| s.v_tsquery.ts_and(s.v_tsquery))
        .into()
}

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsrange.overlaps(s.v_tsrange))
        .into()
}
