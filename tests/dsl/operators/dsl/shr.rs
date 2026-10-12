#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn bit_int4() -> Query<(), (t::Nullable<t::Bit>,)> {
    type_samples.select(|s| s.v_bit >> s.v_int4).into()
}

pub fn box_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_box.right_of(s.v_box)).into()
}

pub fn circle_circle() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_circle.right_of(s.v_circle))
        .into()
}

pub fn inet_inet() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_inet.subnet_gt(s.v_inet)).into()
}

pub fn int2_int4() -> Query<(), (t::Nullable<t::Int2>,)> {
    type_samples.select(|s| s.v_int2 >> s.v_int4).into()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int4 >> s.v_int4).into()
}

pub fn int4multirange_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4multirange.right_of(s.v_int4multirange))
        .into()
}

pub fn int4multirange_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4multirange.right_of(s.v_int4range))
        .into()
}

pub fn int4range_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4range.right_of(s.v_int4multirange))
        .into()
}

pub fn int4range_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4range.right_of(s.v_int4range))
        .into()
}

pub fn int8_int4() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8 >> s.v_int4).into()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_point.right_of(s.v_point))
        .into()
}

pub fn polygon_polygon() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_polygon.right_of(s.v_polygon))
        .into()
}

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsrange.right_of(s.v_tsrange))
        .into()
}
