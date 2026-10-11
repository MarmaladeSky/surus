use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn box_point() -> Query<(), (t::Nullable<t::PgBox>,)> {
    placeholder()
}

pub fn circle_point() -> Query<(), (t::Nullable<t::Circle>,)> {
    placeholder()
}

pub fn float4_float4() -> Query<(), (t::Nullable<t::Float4>,)> {
    placeholder()
}

pub fn float4_float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}

pub fn float8_float4() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}

pub fn float8_float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}

pub fn int2_int2() -> Query<(), (t::Nullable<t::Int2>,)> {
    placeholder()
}

pub fn int2_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn int2_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn int4_int2() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn int4_int4() -> Query<(), (t::Nullable<t::Int4>,)> {
    placeholder()
}

pub fn int4_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn int8_int2() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn int8_int4() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn int8_int8() -> Query<(), (t::Nullable<t::Int8>,)> {
    placeholder()
}

pub fn interval_float8() -> Query<(), (t::Nullable<t::Interval>,)> {
    placeholder()
}

pub fn money_float4() -> Query<(), (t::Nullable<t::Money>,)> {
    placeholder()
}

pub fn money_float8() -> Query<(), (t::Nullable<t::Money>,)> {
    placeholder()
}

pub fn money_int2() -> Query<(), (t::Nullable<t::Money>,)> {
    placeholder()
}

pub fn money_int4() -> Query<(), (t::Nullable<t::Money>,)> {
    placeholder()
}

pub fn money_int8() -> Query<(), (t::Nullable<t::Money>,)> {
    placeholder()
}

pub fn money_money() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}

pub fn numeric_numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    placeholder()
}

pub fn path_point() -> Query<(), (t::Nullable<t::Path>,)> {
    placeholder()
}

pub fn point_point() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}
