#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn bit() -> Query<(), (t::Nullable<t::Bit>,)> {
    type_samples.select(|s| !s.v_bit).into()
}

pub fn bpchar_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_bpchar.regex(s.v_text)).into()
}

pub fn inet() -> Query<(), (t::Nullable<t::Inet>,)> {
    type_samples.select(|s| !s.v_inet).into()
}

pub fn int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    type_samples.select(|s| !s.v_int2).into()
}

pub fn int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| !s.v_int4).into()
}

pub fn int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| !s.v_int8).into()
}

pub fn macaddr() -> Query<(), (t::Nullable<t::Macaddr>,)> {
    type_samples.select(|s| !s.v_macaddr).into()
}

pub fn macaddr8() -> Query<(), (t::Nullable<t::Macaddr8>,)> {
    type_samples.select(|s| !s.v_macaddr8).into()
}

pub fn text_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_text.regex(s.v_text)).into()
}
