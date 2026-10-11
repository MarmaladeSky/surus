use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn bit_bit() -> Query<(), (t::Nullable<t::Bit>,)> {
    placeholder()
}

pub fn inet_inet() -> Query<(), (t::Nullable<t::Inet>,)> {
    placeholder()
}

pub fn int2_int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    placeholder()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn macaddr8_macaddr8() -> Query<(), (t::Nullable<t::Macaddr8>,)> {
    placeholder()
}

pub fn macaddr_macaddr() -> Query<(), (t::Nullable<t::Macaddr>,)> {
    placeholder()
}
