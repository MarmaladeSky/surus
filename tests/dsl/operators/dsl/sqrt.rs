#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    type_samples.select(|s| s.v_float8.sqrt()).into()
}
