#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn price_tag_price_tag() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_price_tag.image_eq(s.v_price_tag))
        .into()
}
