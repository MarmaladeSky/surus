#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn jsonb_jsonpath() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_jsonb.path_exists(s.v_jsonpath))
        .into()
}
