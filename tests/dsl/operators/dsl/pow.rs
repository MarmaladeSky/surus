use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn float8_float8() -> Query<(), (t::Nullable<t::Float8>,)> {
    placeholder()
}

pub fn numeric_numeric() -> Query<(), (t::Nullable<t::Numeric>,)> {
    placeholder()
}
