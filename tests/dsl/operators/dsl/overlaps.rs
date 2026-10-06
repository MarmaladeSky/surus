// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn box_box() -> Query {
    t.select(t.v_box.overlaps(t.v_box)).compile().into()
}

pub fn circle_circle() -> Query {
    t.select(t.v_circle.overlaps(t.v_circle)).compile().into()
}

pub fn inet_inet() -> Query {
    t.select(t.v_inet.overlaps(t.v_inet)).compile().into()
}

pub fn int4_array_int4_array() -> Query {
    t.select(t.v_int4_array.overlaps(t.v_int4_array))
        .compile()
        .into()
}

pub fn int4multirange_int4multirange() -> Query {
    t.select(t.v_int4multirange.overlaps(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4multirange_int4range() -> Query {
    t.select(t.v_int4multirange.overlaps(t.v_int4range))
        .compile()
        .into()
}

pub fn int4range_int4multirange() -> Query {
    t.select(t.v_int4range.overlaps(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4range_int4range() -> Query {
    t.select(t.v_int4range.overlaps(t.v_int4range))
        .compile()
        .into()
}

pub fn polygon_polygon() -> Query {
    t.select(t.v_polygon.overlaps(t.v_polygon)).compile().into()
}

pub fn text_array_text_array() -> Query {
    t.select(t.v_text_array.overlaps(t.v_text_array))
        .compile()
        .into()
}

pub fn tsquery_tsquery() -> Query {
    t.select(t.v_tsquery.overlaps(t.v_tsquery)).compile().into()
}

pub fn tsrange_tsrange() -> Query {
    t.select(t.v_tsrange.overlaps(t.v_tsrange)).compile().into()
}
