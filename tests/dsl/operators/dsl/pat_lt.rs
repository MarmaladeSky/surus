#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn bpchar_bpchar() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_bpchar.pat_lt(s.v_bpchar))
        .into()
}

pub fn text_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_text.pat_lt(s.v_text)).into()
}
