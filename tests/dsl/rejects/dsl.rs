use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Int4, Int8, Nullable, Text};

pub fn column_existence() -> Query<(), (Text,)> {
    placeholder()
}

pub fn column_scope() -> Query<(), (Text,)> {
    placeholder()
}

pub fn where_boolean() -> Query<(), (Text,)> {
    placeholder()
}

pub fn having_boolean() -> Query<(), (Nullable<Int8>, Int8)> {
    placeholder()
}

pub fn join_on_boolean() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn update_where_boolean() -> Query<(), (Text,)> {
    placeholder()
}

pub fn delete_where_boolean() -> Query<(), (Text,)> {
    placeholder()
}

pub fn case_when_boolean() -> Query<(), (Nullable<Text>,)> {
    placeholder()
}

pub fn filter_boolean() -> Query<(), (Int8,)> {
    placeholder()
}

pub fn comparison_types() -> Query<(), (Text,)> {
    placeholder()
}

pub fn operator_operands() -> Query<(), (Nullable<Bool>,)> {
    placeholder()
}

pub fn function_arguments() -> Query<(), (Text,)> {
    placeholder()
}

pub fn cast_validity() -> Query<(), (Text,)> {
    placeholder()
}

pub fn parameter_mapping() -> Query<(Int8,), (Text,)> {
    placeholder()
}

pub fn insert_value_type() -> Query<(), (Text,)> {
    placeholder()
}

pub fn insert_not_null() -> Query<(), (Text,)> {
    placeholder()
}

pub fn insert_arity() -> Query<(), (Text,)> {
    placeholder()
}

pub fn update_assignment_type() -> Query<(), (Text, Int4)> {
    placeholder()
}

pub fn nullability_propagation() -> Query<(), (Text,)> {
    placeholder()
}

pub fn result_row_type() -> Query<(), (Int8, Text)> {
    placeholder()
}

pub fn returning_type() -> Query<(), (Bool, Text)> {
    placeholder()
}

pub fn cte_column_type() -> Query<(), (Text,)> {
    placeholder()
}

pub fn union_column_types() -> Query<(), (Int8, Text)> {
    placeholder()
}

pub fn union_arity() -> Query<(), (Int8, Text)> {
    placeholder()
}

pub fn scalar_subquery_shape() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn in_subquery_type() -> Query<(), (Text,)> {
    placeholder()
}
