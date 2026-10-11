use crate::support::placeholder;
use surus::Query;
use surus::types::{Int8, Nullable, Text};

pub fn scalar_group_name() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn scalar_multiple_rows_fails() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn exists_groups_with_users() -> Query<(), (Text,)> {
    placeholder()
}

pub fn not_exists_empty_groups() -> Query<(), (Text,)> {
    placeholder()
}

pub fn in_subquery() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn any_not_the_cheapest() -> Query<(), (Text,)> {
    placeholder()
}

pub fn all_the_most_expensive() -> Query<(), (Text,)> {
    placeholder()
}

pub fn not_in_subquery() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn not_in_with_null_returns_nothing() -> Query<(), (Text, Int8)> {
    placeholder()
}

pub fn row_valued_in() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn correlated_scalar_in_arithmetic() -> Query<(), (Text, Nullable<Int8>)> {
    placeholder()
}

pub fn exists_and_predicate() -> Query<(), (Text,)> {
    placeholder()
}

pub fn scalar_subquery_compared() -> Query<(), (Text,)> {
    placeholder()
}
