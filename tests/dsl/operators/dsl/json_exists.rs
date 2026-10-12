#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn jsonb_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_jsonb.has_key(s.v_text)).into()
}
