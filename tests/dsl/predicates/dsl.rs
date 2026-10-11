use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Int4, Int8, Nullable, Numeric, Text};

pub fn and_or_not() -> Query<(), (Text,)> {
    placeholder()
}

pub fn is_null() -> Query<(), (Text, Bool, Bool)> {
    placeholder()
}

pub fn is_distinct_from() -> Query<(), (Text, Text, Bool)> {
    placeholder()
}

pub fn is_true() -> Query<(), (Text, Bool, Bool, Bool)> {
    placeholder()
}

pub fn between(_low: i32, _high: i32) -> Query<(), (Text,)> {
    placeholder()
}

pub fn in_list() -> Query<(Text, Text), (Text,)> {
    placeholder()
}

pub fn like(_pattern: &str) -> Query<(), (Text,)> {
    placeholder()
}

pub fn ilike(_pattern: &str) -> Query<(), (Text,)> {
    placeholder()
}

pub fn similar_to(_pattern: &str) -> Query<(), (Text,)> {
    placeholder()
}

pub fn like_escape(_pattern: &str, _escape: &str) -> Query<(), (Text,)> {
    placeholder()
}

pub fn between_symmetric(_first: i32, _second: i32) -> Query<(), (Text,)> {
    placeholder()
}

pub fn overlaps() -> Query<(), (Nullable<Bool>, Nullable<Bool>, Nullable<Bool>)> {
    placeholder()
}

pub fn arithmetic_chain() -> Query<(), (Text, Int4)> {
    placeholder()
}

pub fn arithmetic_nullable_column() -> Query<(), (Text, Nullable<Int8>)> {
    placeholder()
}

pub fn arithmetic_mixed_nullability() -> Query<(), (Text, Nullable<Int8>)> {
    placeholder()
}

pub fn negate_expression() -> Query<(), (Text, Int4)> {
    placeholder()
}

pub fn compare_expressions() -> Query<(), (Text,)> {
    placeholder()
}

pub fn bool_mixed_nullability() -> Query<(), (Text,)> {
    placeholder()
}

pub fn operator_results_combined() -> Query<(), (Nullable<Int4>, Nullable<Bool>)> {
    placeholder()
}

pub fn json_chain_into_text_op() -> Query<(), (Nullable<Text>,)> {
    placeholder()
}

pub fn cast_of_expression() -> Query<(), (Text, Numeric)> {
    placeholder()
}
