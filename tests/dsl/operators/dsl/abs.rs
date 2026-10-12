#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn float4() -> Query<(), (t::Nullable<t::Float4>,)> {
    type_samples.select(|s| s.v_float4.abs()).into()
}

pub fn float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_float8.abs()).into()
}

pub fn int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    type_samples.select(|s| s.v_int2.abs()).into()
}

pub fn int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int4.abs()).into()
}

pub fn int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8.abs()).into()
}

pub fn numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    type_samples.select(|s| s.v_numeric.abs()).into()
}
