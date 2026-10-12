#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn inet_inet() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_inet.subnet_ge(s.v_inet)).into()
}
