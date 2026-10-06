// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn box_box() -> Query {
    t.select(t.v_box.contained_by(t.v_box)).compile().into()
}

pub fn circle_circle() -> Query {
    t.select(t.v_circle.contained_by(t.v_circle))
        .compile()
        .into()
}

pub fn int4_array_int4_array() -> Query {
    t.select(t.v_int4_array.contained_by(t.v_int4_array))
        .compile()
        .into()
}

pub fn int4_int4multirange() -> Query {
    t.select(t.v_int4.contained_by(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4_int4range() -> Query {
    t.select(t.v_int4.contained_by(t.v_int4range))
        .compile()
        .into()
}

pub fn int4multirange_int4multirange() -> Query {
    t.select(t.v_int4multirange.contained_by(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4multirange_int4range() -> Query {
    t.select(t.v_int4multirange.contained_by(t.v_int4range))
        .compile()
        .into()
}

pub fn int4range_int4multirange() -> Query {
    t.select(t.v_int4range.contained_by(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4range_int4range() -> Query {
    t.select(t.v_int4range.contained_by(t.v_int4range))
        .compile()
        .into()
}

pub fn jsonb_jsonb() -> Query {
    t.select(t.v_jsonb.contained_by(t.v_jsonb)).compile().into()
}

pub fn lseg_box() -> Query {
    t.select(t.v_lseg.contained_by(t.v_box)).compile().into()
}

pub fn lseg_line() -> Query {
    t.select(t.v_lseg.contained_by(t.v_line)).compile().into()
}

pub fn point_box() -> Query {
    t.select(t.v_point.contained_by(t.v_box)).compile().into()
}

pub fn point_circle() -> Query {
    t.select(t.v_point.contained_by(t.v_circle))
        .compile()
        .into()
}

pub fn point_line() -> Query {
    t.select(t.v_point.contained_by(t.v_line)).compile().into()
}

pub fn point_lseg() -> Query {
    t.select(t.v_point.contained_by(t.v_lseg)).compile().into()
}

pub fn point_path() -> Query {
    t.select(t.v_point.contained_by(t.v_path)).compile().into()
}

pub fn point_polygon() -> Query {
    t.select(t.v_point.contained_by(t.v_polygon))
        .compile()
        .into()
}

pub fn polygon_polygon() -> Query {
    t.select(t.v_polygon.contained_by(t.v_polygon))
        .compile()
        .into()
}

pub fn text_array_text_array() -> Query {
    t.select(t.v_text_array.contained_by(t.v_text_array))
        .compile()
        .into()
}

pub fn timestamp_tsrange() -> Query {
    t.select(t.v_timestamp.contained_by(t.v_tsrange))
        .compile()
        .into()
}

pub fn tsquery_tsquery() -> Query {
    t.select(t.v_tsquery.contained_by(t.v_tsquery))
        .compile()
        .into()
}

pub fn tsrange_tsrange() -> Query {
    t.select(t.v_tsrange.contained_by(t.v_tsrange))
        .compile()
        .into()
}
