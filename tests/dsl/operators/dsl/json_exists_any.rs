// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn jsonb_text_array() -> Query {
    t.select(t.v_jsonb.has_any_key(t.v_text_array))
        .compile()
        .into()
}

pub fn line() -> Query {
    t.select(t.v_line.is_vertical()).compile().into()
}

pub fn lseg() -> Query {
    t.select(t.v_lseg.is_vertical()).compile().into()
}

pub fn point_point() -> Query {
    t.select(t.v_point.vertical(t.v_point)).compile().into()
}
