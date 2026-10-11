use crate::schema::PositiveInt;
use crate::support::placeholder;
use surus::Query;
use surus::types::{Nullable, Regtype};

pub fn cast_to_domain() -> Query<(), (PositiveInt, PositiveInt, Regtype)> {
    placeholder()
}

pub fn check_violation_on_cast() -> Query<(), (PositiveInt,)> {
    placeholder()
}

pub fn check_violation_on_insert(_value: i32) -> Query<(), (Nullable<PositiveInt>,)> {
    placeholder()
}
