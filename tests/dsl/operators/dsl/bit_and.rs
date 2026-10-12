#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn bit_bit() -> Query<(), (t::Nullable<t::Bit>,)> {
    type_samples.select(|s| s.v_bit & s.v_bit).into()
}

pub fn inet_inet() -> Query<(), (t::Nullable<t::Inet>,)> {
    type_samples.select(|s| s.v_inet & s.v_inet).into()
}

pub fn int2_int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    type_samples.select(|s| s.v_int2 & s.v_int2).into()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    type_samples.select(|s| s.v_int4 & s.v_int4).into()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    type_samples.select(|s| s.v_int8 & s.v_int8).into()
}

pub fn macaddr8_macaddr8() -> Query<(), (t::Nullable<t::Macaddr8>,)> {
    type_samples.select(|s| s.v_macaddr8 & s.v_macaddr8).into()
}

pub fn macaddr_macaddr() -> Query<(), (t::Nullable<t::Macaddr>,)> {
    type_samples.select(|s| s.v_macaddr & s.v_macaddr).into()
}
