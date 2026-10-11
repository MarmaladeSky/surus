use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn bit_bit() -> Query<(), (t::Nullable<t::Bit>,)> {
    placeholder()
}

pub fn box_box() -> Query<(), (t::Nullable<t::PgBox>,)> {
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

pub fn line_line() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn lseg_lseg() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn path() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn polygon() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}
