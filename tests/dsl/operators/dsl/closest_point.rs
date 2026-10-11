use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn line_lseg() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn lseg_box() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn lseg_lseg() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn point_box() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn point_line() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn point_lseg() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}
