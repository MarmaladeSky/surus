use crate::support::placeholder;
use surus::Query;
use surus::types::{Array, Int4, Text};

pub fn named_users() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn recursive_ancestors() -> Query<(Text,), (Text, Int4)> {
    placeholder()
}

pub fn recursive_descendants_with_path() -> Query<(Text,), (Array<Text>,)> {
    placeholder()
}

pub fn data_modifying() -> Query<(Text, Text), (Text, Text)> {
    placeholder()
}

pub fn chained_materialized() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn cte_column_expressions() -> Query<(), (Text, Int4)> {
    placeholder()
}

pub fn cte_joined_to_table() -> Query<(), (Text,)> {
    placeholder()
}

pub fn cte_referenced_twice() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn cte_in_subquery() -> Query<(), (Text,)> {
    placeholder()
}

pub fn recursive_depth_expression() -> Query<(), (Text, Int4)> {
    placeholder()
}

pub fn modifying_cte_returning_used() -> Query<(Text,), (Text,)> {
    placeholder()
}
