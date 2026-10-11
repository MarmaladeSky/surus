use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Int4, Int8, Nullable, Text};

pub fn user_returning() -> Query<(Text, Text), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn product() -> Query<(Text, Int4, Int4), (Text, Int4, Int4, Nullable<Int8>)> {
    placeholder()
}

pub fn insert_select() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn multi_row_values() -> Query<(Text, Text, Text), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn default_value() -> Query<(Text, Int4), (Text, Int4, Int4)> {
    placeholder()
}

pub fn overriding_system_value() -> Query<(Int8, Text), (Int8, Text)> {
    placeholder()
}

pub fn on_conflict_do_nothing() -> Query<(Text, Text), (Int8, Text)> {
    placeholder()
}

pub fn on_conflict_do_update(_theme: &str) -> Query<(Text,), (Int8, Text)> {
    placeholder()
}

pub fn default_values() -> Query<(), (Bool, Bool)> {
    placeholder()
}

pub fn virtual_generated_column() -> Query<(Text, Int4), (Int4, Nullable<Int8>, Nullable<Int8>)> {
    placeholder()
}

pub fn on_conflict_partial_index() -> Query<(Text, Text), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn returning_expression() -> Query<(Text, Int4, Int4), (Int4,)> {
    placeholder()
}

pub fn insert_select_expression() -> Query<(), (Text,)> {
    placeholder()
}

pub fn insert_bound() -> Query<(Text, Nullable<Text>), (Text, Nullable<Text>)> {
    placeholder()
}
