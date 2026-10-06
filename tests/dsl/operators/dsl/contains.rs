// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn box_box() -> Query {
    t.select(t.v_box.contains(t.v_box)).compile().into()
}

pub fn box_point() -> Query {
    t.select(t.v_box.contains(t.v_point)).compile().into()
}

pub fn circle_circle() -> Query {
    t.select(t.v_circle.contains(t.v_circle)).compile().into()
}

pub fn circle_point() -> Query {
    t.select(t.v_circle.contains(t.v_point)).compile().into()
}

pub fn int4_array_int4_array() -> Query {
    t.select(t.v_int4_array.contains(t.v_int4_array))
        .compile()
        .into()
}

pub fn int4multirange_int4() -> Query {
    t.select(t.v_int4multirange.contains(t.v_int4))
        .compile()
        .into()
}

pub fn int4multirange_int4multirange() -> Query {
    t.select(t.v_int4multirange.contains(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4multirange_int4range() -> Query {
    t.select(t.v_int4multirange.contains(t.v_int4range))
        .compile()
        .into()
}

pub fn int4range_int4() -> Query {
    t.select(t.v_int4range.contains(t.v_int4)).compile().into()
}

pub fn int4range_int4multirange() -> Query {
    t.select(t.v_int4range.contains(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4range_int4range() -> Query {
    t.select(t.v_int4range.contains(t.v_int4range))
        .compile()
        .into()
}

pub fn jsonb_jsonb() -> Query {
    t.select(t.v_jsonb.contains(t.v_jsonb)).compile().into()
}

pub fn path_point() -> Query {
    t.select(t.v_path.contains(t.v_point)).compile().into()
}

pub fn polygon_point() -> Query {
    t.select(t.v_polygon.contains(t.v_point)).compile().into()
}

pub fn polygon_polygon() -> Query {
    t.select(t.v_polygon.contains(t.v_polygon)).compile().into()
}

pub fn text_array_text_array() -> Query {
    t.select(t.v_text_array.contains(t.v_text_array))
        .compile()
        .into()
}

pub fn tsquery_tsquery() -> Query {
    t.select(t.v_tsquery.contains(t.v_tsquery)).compile().into()
}

pub fn tsrange_timestamp() -> Query {
    t.select(t.v_tsrange.contains(t.v_timestamp))
        .compile()
        .into()
}

pub fn tsrange_tsrange() -> Query {
    t.select(t.v_tsrange.contains(t.v_tsrange)).compile().into()
}
