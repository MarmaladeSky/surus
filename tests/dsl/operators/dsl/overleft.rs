// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn box_box() -> Query {
    t.select(t.v_box.overleft(t.v_box)).compile().into()
}

pub fn circle_circle() -> Query {
    t.select(t.v_circle.overleft(t.v_circle)).compile().into()
}

pub fn int4multirange_int4multirange() -> Query {
    t.select(t.v_int4multirange.overleft(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4multirange_int4range() -> Query {
    t.select(t.v_int4multirange.overleft(t.v_int4range))
        .compile()
        .into()
}

pub fn int4range_int4multirange() -> Query {
    t.select(t.v_int4range.overleft(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4range_int4range() -> Query {
    t.select(t.v_int4range.overleft(t.v_int4range))
        .compile()
        .into()
}

pub fn polygon_polygon() -> Query {
    t.select(t.v_polygon.overleft(t.v_polygon)).compile().into()
}

pub fn tsrange_tsrange() -> Query {
    t.select(t.v_tsrange.overleft(t.v_tsrange)).compile().into()
}
