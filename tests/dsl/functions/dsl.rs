use crate::support::placeholder;
use surus::Query;
use surus::types::{Date, Float8, Int4, Int8, Interval, Nullable, Numeric, Text, Timestamp};

pub fn string_functions() -> Query<(), (Text, Int4, Text, Text, Text)> {
    placeholder()
}

pub fn math_functions() -> Query<(), (Int4, Numeric, Float8, Int4)> {
    placeholder()
}

pub fn datetime_functions() -> Query<(), (Nullable<Timestamp>, Nullable<Numeric>, Nullable<Text>)> {
    placeholder()
}

pub fn named_argument_notation() -> Query<(), (Interval, Date)> {
    placeholder()
}

pub fn function_of_function() -> Query<(), (Text,)> {
    placeholder()
}

pub fn aggregate_of_expression() -> Query<(), (Int8, Nullable<Int8>)> {
    placeholder()
}

pub fn window_over_expression() -> Query<(), (Text, Nullable<Int8>)> {
    placeholder()
}

pub fn split_part_domain() -> Query<(), (Text, Nullable<Text>)> {
    placeholder()
}
