// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn bit_int4() -> Query {
    t.select(t.v_bit >> t.v_int4).compile().into()
}

pub fn box_box() -> Query {
    t.select(t.v_box >> t.v_box).compile().into()
}

pub fn circle_circle() -> Query {
    t.select(t.v_circle >> t.v_circle).compile().into()
}

pub fn inet_inet() -> Query {
    t.select(t.v_inet >> t.v_inet).compile().into()
}

pub fn int2_int4() -> Query {
    t.select(t.v_int2 >> t.v_int4).compile().into()
}

pub fn int4_int4() -> Query {
    t.select(t.v_int4 >> t.v_int4).compile().into()
}

pub fn int4multirange_int4multirange() -> Query {
    t.select(t.v_int4multirange >> t.v_int4multirange)
        .compile()
        .into()
}

pub fn int4multirange_int4range() -> Query {
    t.select(t.v_int4multirange >> t.v_int4range)
        .compile()
        .into()
}

pub fn int4range_int4multirange() -> Query {
    t.select(t.v_int4range >> t.v_int4multirange)
        .compile()
        .into()
}

pub fn int4range_int4range() -> Query {
    t.select(t.v_int4range >> t.v_int4range).compile().into()
}

pub fn int8_int4() -> Query {
    t.select(t.v_int8 >> t.v_int4).compile().into()
}

pub fn point_point() -> Query {
    t.select(t.v_point >> t.v_point).compile().into()
}

pub fn polygon_polygon() -> Query {
    t.select(t.v_polygon >> t.v_polygon).compile().into()
}

pub fn tsrange_tsrange() -> Query {
    t.select(t.v_tsrange >> t.v_tsrange).compile().into()
}
