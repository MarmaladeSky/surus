use crate::support::placeholder;
use surus::Query;
use surus::types::{Int4, Nullable, Text};

pub fn case_searched() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn case_simple() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn coalesce() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn nullif() -> Query<(), (Text, Nullable<Int4>)> {
    placeholder()
}

pub fn greatest_least() -> Query<(), (Text, Int4, Int4)> {
    placeholder()
}

pub fn case_into_arithmetic() -> Query<(), (Text, Int4)> {
    placeholder()
}
