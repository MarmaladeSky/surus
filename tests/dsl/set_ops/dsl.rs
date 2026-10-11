use crate::support::placeholder;
use surus::Query;
use surus::types::{Int8, Nullable, Text};

pub fn user_and_group_names() -> Query<(), (Text,)> {
    placeholder()
}

pub fn union_all_keeps_duplicates() -> Query<(), (Text,)> {
    placeholder()
}

pub fn intersect() -> Query<(), (Text,)> {
    placeholder()
}

pub fn except() -> Query<(), (Text,)> {
    placeholder()
}

pub fn parenthesized_with_limits() -> Query<(), (Text,)> {
    placeholder()
}

pub fn union_nullable_with_not_null() -> Query<(), (Nullable<Text>,)> {
    placeholder()
}

pub fn set_op_in_from() -> Query<(), (Int8,)> {
    placeholder()
}

pub fn set_op_as_cte() -> Query<(), (Text,)> {
    placeholder()
}

pub fn nested_precedence() -> Query<(), (Nullable<Text>,)> {
    placeholder()
}
