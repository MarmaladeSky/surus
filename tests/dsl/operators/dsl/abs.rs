use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn float4() -> Query<(), (t::Nullable<t::Float4>,)> {
    placeholder()
}

pub fn float8() -> Query<(), (t::Nullable<t::Float8>,)> {
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

pub fn numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    placeholder()
}
