use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Int4, Int8, Nullable, Text};

pub fn set_email() -> Query<(Text, Text), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn update_from() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn set_row_from_subquery() -> Query<(Text, Text), (Text, Nullable<Text>, Bool)> {
    placeholder()
}

pub fn set_row_constructor() -> Query<(Text, Text, Text), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn returning_star() -> Query<(Text,), (Int8, Text, Int4, Int4, Nullable<Int8>, Nullable<Int8>)>
{
    placeholder()
}

pub fn returning_old_and_new(_delta: i32) -> Query<(Text,), (Int4, Int4, Nullable<Int8>)> {
    placeholder()
}

pub fn update_set_expression() -> Query<(), (Text, Int4)> {
    placeholder()
}

pub fn update_from_join_predicate() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn update_bound() -> Query<(Text, Int8), (Int8, Text)> {
    placeholder()
}
