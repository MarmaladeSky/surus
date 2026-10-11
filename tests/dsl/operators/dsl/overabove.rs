use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn box_box() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn circle_circle() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn polygon_polygon() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
