use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn box_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn circle_circle() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn inet_inet() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn int4_array_int4_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn int4multirange_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn int4multirange_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn int4range_int4multirange() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn int4range_int4range() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn polygon_polygon() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn text_array_text_array() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn tsquery_tsquery() -> Query<(), (t::Nullable<t::Tsquery>,)> {
    placeholder()
}

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
