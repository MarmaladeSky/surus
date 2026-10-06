// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn price_tag_price_tag() -> Query {
    t.select(t.v_price_tag.rec_ge(t.v_price_tag))
        .compile()
        .into()
}
