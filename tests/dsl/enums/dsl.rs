use crate::schema::Mood;
use crate::support::placeholder;
use surus::Query;
use surus::types::{Array, Nullable};

pub fn declaration_order() -> Query<(), (Nullable<Mood>,)> {
    placeholder()
}

pub fn comparison_with_parameter(_mood: &str) -> Query<(), (Nullable<Mood>,)> {
    placeholder()
}

pub fn enum_functions() -> Query<
    (),
    (
        Nullable<Mood>,
        Nullable<Array<Mood>>,
        Nullable<Mood>,
        Nullable<Mood>,
        Nullable<Array<Mood>>,
    ),
> {
    placeholder()
}
