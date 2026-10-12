#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn json_int4() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_json.get_text(s.v_int4)).into()
}

pub fn json_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_json.get_text(s.v_text)).into()
}

pub fn jsonb_int4() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_jsonb.get_text(s.v_int4)).into()
}

pub fn jsonb_text() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples.select(|s| s.v_jsonb.get_text(s.v_text)).into()
}
