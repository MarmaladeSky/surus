use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn int2_int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    placeholder()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn numeric_numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    placeholder()
}
