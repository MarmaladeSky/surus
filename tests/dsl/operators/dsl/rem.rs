#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn int2_int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    type_samples.select(|s| s.v_int2 % s.v_int2).into()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int4 % s.v_int4).into()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8 % s.v_int8).into()
}

pub fn numeric_numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    type_samples.select(|s| s.v_numeric % s.v_numeric).into()
}
