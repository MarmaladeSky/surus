use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn line() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn lseg() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
