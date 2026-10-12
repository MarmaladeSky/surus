#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn tsquery_tsvector() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsquery.matches_alt(s.v_tsvector))
        .into()
}

pub fn tsvector_tsquery() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsvector.matches_alt(s.v_tsquery))
        .into()
}
