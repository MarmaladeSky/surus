#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn int4multirange_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4multirange.adjacent(s.v_int4multirange))
        .into()
}

pub fn int4multirange_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4multirange.adjacent(s.v_int4range))
        .into()
}

pub fn int4range_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4range.adjacent(s.v_int4multirange))
        .into()
}

pub fn int4range_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_int4range.adjacent(s.v_int4range))
        .into()
}

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsrange.adjacent(s.v_tsrange))
        .into()
}
