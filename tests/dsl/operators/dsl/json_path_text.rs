// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn json_text_array() -> Query {
    t.select(t.v_json.get_path_text(t.v_text_array))
        .compile()
        .into()
}

pub fn jsonb_text_array() -> Query {
    t.select(t.v_jsonb.get_path_text(t.v_text_array))
        .compile()
        .into()
}
