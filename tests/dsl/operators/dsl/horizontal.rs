#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn line() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_line.is_horizontal()).into()
}

pub fn lseg() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_lseg.is_horizontal()).into()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_point.horizontally_aligned(s.v_point))
        .into()
}
