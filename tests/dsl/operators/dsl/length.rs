use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn lseg() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}

pub fn path() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}
