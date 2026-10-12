#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn bit_bit() -> Query<(), (t::Nullable<t::Bit>,)> {
    type_samples.select(|s| s.v_bit ^ s.v_bit).into()
}

pub fn box_box() -> Query<(), (t::Nullable<t::PgBox>,)> {
    type_samples
        .select(|s| s.v_box.intersection(s.v_box))
        .into()
}

pub fn int2_int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    type_samples.select(|s| s.v_int2 ^ s.v_int2).into()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int4 ^ s.v_int4).into()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8 ^ s.v_int8).into()
}

pub fn line_line() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples
        .select(|s| s.v_line.intersection(s.v_line))
        .into()
}

pub fn lseg_lseg() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples
        .select(|s| s.v_lseg.intersection(s.v_lseg))
        .into()
}

pub fn path() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_path.npoints()).into()
}

pub fn polygon() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_polygon.npoints()).into()
}
