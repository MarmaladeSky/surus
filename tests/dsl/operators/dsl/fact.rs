#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn tsquery() -> Query<(), (t::Nullable<t::Tsquery>,)> {
    type_samples.select(|s| s.v_tsquery.negate()).into()
}
