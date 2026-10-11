use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn bit() -> Query<(), (t::Nullable<t::Bit>,)> {
    placeholder()
}

pub fn bpchar_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn inet() -> Query<(), (t::Nullable<t::Inet>,)> {
    placeholder()
}

pub fn int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    placeholder()
}

pub fn int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn macaddr() -> Query<(), (t::Nullable<t::Macaddr>,)> {
    placeholder()
}

pub fn macaddr8() -> Query<(), (t::Nullable<t::Macaddr8>,)> {
    placeholder()
}

pub fn text_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
