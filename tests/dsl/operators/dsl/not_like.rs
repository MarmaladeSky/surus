// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn bpchar_text() -> Query {
    t.select(t.v_bpchar.not_like(t.v_text)).compile().into()
}

pub fn bytea_bytea() -> Query {
    t.select(t.v_bytea.not_like(t.v_bytea)).compile().into()
}

pub fn text_text() -> Query {
    t.select(t.v_text.not_like(t.v_text)).compile().into()
}
