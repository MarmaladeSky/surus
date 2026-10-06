// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn bpchar_bpchar() -> Query {
    t.select(t.v_bpchar.pattern_lt(t.v_bpchar)).compile().into()
}

pub fn text_text() -> Query {
    t.select(t.v_text.pattern_lt(t.v_text)).compile().into()
}
