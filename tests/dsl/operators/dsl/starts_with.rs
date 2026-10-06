// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn text_text() -> Query {
    t.select(t.v_text.starts_with(t.v_text)).compile().into()
}
