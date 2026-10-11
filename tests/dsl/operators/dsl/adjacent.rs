use crate::support::placeholder;
use surus::Query;
use surus::types as t;

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

pub fn tsrange_tsrange() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
