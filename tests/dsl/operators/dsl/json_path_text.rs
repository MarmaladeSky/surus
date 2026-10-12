#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn json_text_array() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_json.get_path_text(s.v_text_array))
        .into()
}

pub fn jsonb_text_array() -> Query<(), (t::Nullable<t::Text>,)> {
    type_samples
        .select(|s| s.v_jsonb.get_path_text(s.v_text_array))
        .into()
}
