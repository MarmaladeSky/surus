use crate::support::placeholder;
use surus::Query;
use surus::types::{Float8, Int4, Int8, Nullable, Numeric, Text};

pub fn row_number_by_name() -> Query<(), (Text, Int8)> {
    placeholder()
}

pub fn partition_by() -> Query<(), (Text, Int4, Int8, Nullable<Int8>, Int8)> {
    placeholder()
}

pub fn lag_lead() -> Query<(), (Text, Nullable<Int4>, Nullable<Int4>)> {
    placeholder()
}

pub fn frame_clauses() -> Query<(), (Text, Nullable<Int8>, Nullable<Numeric>, Int8)> {
    placeholder()
}

pub fn named_window() -> Query<(), (Text, Int8, Int8, Int4)> {
    placeholder()
}

pub fn value_functions() -> Query<
    (),
    (
        Text,
        Nullable<Text>,
        Nullable<Text>,
        Nullable<Text>,
        Float8,
        Float8,
    ),
> {
    placeholder()
}

pub fn frame_exclusion_and_offsets() -> Query<(), (Text, Nullable<Int8>, Nullable<Int8>, Int8, Int8)>
{
    placeholder()
}
