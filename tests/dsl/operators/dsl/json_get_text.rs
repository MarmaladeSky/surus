// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn json_int4() -> Query {
    t.select(t.v_json.get_text(t.v_int4)).compile().into()
}

pub fn json_text() -> Query {
    t.select(t.v_json.get_text(t.v_text)).compile().into()
}

pub fn jsonb_int4() -> Query {
    t.select(t.v_jsonb.get_text(t.v_int4)).compile().into()
}

pub fn jsonb_text() -> Query {
    t.select(t.v_jsonb.get_text(t.v_text)).compile().into()
}
