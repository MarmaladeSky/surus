#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn line_line() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_line.is_parallel(s.v_line))
        .into()
}

pub fn lseg_lseg() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_lseg.is_parallel(s.v_lseg))
        .into()
}
