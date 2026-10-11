use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Int4, Int8, Nullable, Text};

pub fn users_with_group() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn group_ancestor_30_levels() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn logins_30_day_pivot() -> Query<(), super::Logins> {
    placeholder()
}

pub fn left_join() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn left_join_null_propagation() -> Query<(), (Text, Text, Bool)> {
    placeholder()
}

pub fn right_join() -> Query<(), (Nullable<Text>, Text)> {
    placeholder()
}

pub fn full_join() -> Query<(), (Nullable<Text>, Nullable<Text>)> {
    placeholder()
}

pub fn cross_join() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn natural_join() -> Query<(), (Int8, Text, Text)> {
    placeholder()
}

pub fn join_using() -> Query<(), (Int8, Text, Nullable<Text>)> {
    placeholder()
}

pub fn lateral_first_user_per_group() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn subquery_in_from() -> Query<(), (Text, Int8)> {
    placeholder()
}

pub fn tablesample_repeatable() -> Query<(), (Text,)> {
    placeholder()
}

pub fn with_ordinality() -> Query<(), (Text, Text, Int8)> {
    placeholder()
}

pub fn set_returning_function_in_from(_from: i32, _to: i32) -> Query<(), (Text, Int4)> {
    placeholder()
}

pub fn rows_from_multiple_functions() -> Query<(), (Text, Nullable<Int4>, Nullable<Text>, Int8)> {
    placeholder()
}

pub fn function_columns_in_join_on() -> Query<(Int8, Int8), (Text, Int8)> {
    placeholder()
}

pub fn ordinality_column_in_where() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn values_in_from_joined() -> Query<(Int8, Int8), (Text, Text)> {
    placeholder()
}

pub fn self_join_aliases() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn lateral_columns_in_projection_expr() -> Query<(), (Text, Int8)> {
    placeholder()
}

pub fn subquery_columns_in_where() -> Query<(), (Text, Int4)> {
    placeholder()
}
