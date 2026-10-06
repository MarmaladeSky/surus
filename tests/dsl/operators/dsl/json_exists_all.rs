// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn jsonb_text_array() -> Query {
    t.select(t.v_jsonb.has_all_keys(t.v_text_array))
        .compile()
        .into()
}
