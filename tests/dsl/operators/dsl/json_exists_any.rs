#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn jsonb_text_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_jsonb.has_any_key(s.v_text_array))
        .into()
}

pub fn line() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_line.is_vertical()).into()
}

pub fn lseg() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_lseg.is_vertical()).into()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_point.vertically_aligned(s.v_point))
        .into()
}
