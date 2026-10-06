// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn int4multirange_int4multirange() -> Query {
    t.select(t.v_int4multirange.adjacent(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4multirange_int4range() -> Query {
    t.select(t.v_int4multirange.adjacent(t.v_int4range))
        .compile()
        .into()
}

pub fn int4range_int4multirange() -> Query {
    t.select(t.v_int4range.adjacent(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4range_int4range() -> Query {
    t.select(t.v_int4range.adjacent(t.v_int4range))
        .compile()
        .into()
}

pub fn tsrange_tsrange() -> Query {
    t.select(t.v_tsrange.adjacent(t.v_tsrange)).compile().into()
}
