use crate::support::placeholder;
use surus::Query;
use surus::types::{Int8, Nullable, Text};

pub fn desc() -> Query<(), (Text,)> {
    placeholder()
}

pub fn nulls_first() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn desc_nulls_last() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn multiple_keys() -> Query<(), (Nullable<Int8>, Text)> {
    placeholder()
}

pub fn by_expression() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn by_position() -> Query<(), (Nullable<Int8>, Text)> {
    placeholder()
}

pub fn collate_c() -> Query<(), (Text,)> {
    placeholder()
}

pub fn using_operator() -> Query<(), (Nullable<Int8>, Text)> {
    placeholder()
}
