#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn box_point() -> Query<(), (t::Nullable<t::PgBox>,)> {
    type_samples.select(|s| s.v_box / s.v_point).into()
}

pub fn circle_point() -> Query<(), (t::Nullable<t::Circle>,)> {
    type_samples.select(|s| s.v_circle / s.v_point).into()
}

pub fn float4_float4() -> Query<(), (t::Nullable<t::Float4>,)> {
    type_samples.select(|s| s.v_float4 / s.v_float4).into()
}

pub fn float4_float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_float4 / s.v_float8).into()
}

pub fn float8_float4() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_float8 / s.v_float4).into()
}

pub fn float8_float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_float8 / s.v_float8).into()
}

pub fn int2_int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    type_samples.select(|s| s.v_int2 / s.v_int2).into()
}

pub fn int2_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int2 / s.v_int4).into()
}

pub fn int2_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int2 / s.v_int8).into()
}

pub fn int4_int2() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int4 / s.v_int2).into()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int4 / s.v_int4).into()
}

pub fn int4_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int4 / s.v_int8).into()
}

pub fn int8_int2() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8 / s.v_int2).into()
}

pub fn int8_int4() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8 / s.v_int4).into()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8 / s.v_int8).into()
}

pub fn interval_float8() -> Query<(), (t::Nullable<t::Interval>,)> {
    type_samples.select(|s| s.v_interval / s.v_float8).into()
}

pub fn money_float4() -> Query<(), (t::Nullable<t::Money>,)> {
    type_samples.select(|s| s.v_money / s.v_float4).into()
}

pub fn money_float8() -> Query<(), (t::Nullable<t::Money>,)> {
    type_samples.select(|s| s.v_money / s.v_float8).into()
}

pub fn money_int2() -> Query<(), (t::Nullable<t::Money>,)> {
    type_samples.select(|s| s.v_money / s.v_int2).into()
}

pub fn money_int4() -> Query<(), (t::Nullable<t::Money>,)> {
    type_samples.select(|s| s.v_money / s.v_int4).into()
}

pub fn money_int8() -> Query<(), (t::Nullable<t::Money>,)> {
    type_samples.select(|s| s.v_money / s.v_int8).into()
}

pub fn money_money() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_money / s.v_money).into()
}

pub fn numeric_numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    type_samples.select(|s| s.v_numeric / s.v_numeric).into()
}

pub fn path_point() -> Query<(), (t::Nullable<t::Path>,)> {
    type_samples.select(|s| s.v_path / s.v_point).into()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples.select(|s| s.v_point / s.v_point).into()
}
