use crate::support::placeholder;
use surus::Query;
use surus::types::{Array, Int4, Nullable, Text};

pub fn constructor() -> Query<(), (Array<Int4>, Array<Text>)> {
    placeholder()
}

pub fn subscript_and_slice() -> Query<(), (Nullable<Int4>, Nullable<Array<Int4>>, Nullable<Int4>)> {
    placeholder()
}

pub fn unnest() -> Query<(), (Nullable<Text>,)> {
    placeholder()
}

pub fn any_array_parameter() -> Query<(Array<Int4>,), (Text,)> {
    placeholder()
}
