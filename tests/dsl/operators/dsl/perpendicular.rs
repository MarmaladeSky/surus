use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn line_line() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn lseg_lseg() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
