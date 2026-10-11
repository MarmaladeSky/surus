use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn bit_int4() -> Query<(), (t::Nullable<t::Bit>,)> {
    placeholder()
}

pub fn box_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn circle_circle() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn inet_inet() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn int2_int4() -> Query<(), (t::Nullable<t::Int2>,)> {
    placeholder()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
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

pub fn int8_int4() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn polygon_polygon() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
