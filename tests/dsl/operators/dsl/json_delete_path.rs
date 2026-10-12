#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn jsonb_text_array() -> Query<(), (t::Nullable<t::Jsonb>,)> {
    type_samples
        .select(|s| s.v_jsonb.remove_path(s.v_text_array))
        .into()
}
