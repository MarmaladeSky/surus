use crate::support::placeholder;
use surus::Query;
use surus::types::{Array, Bool, Float8, Int4, Int8, Nullable, Numeric, Text};

pub fn count_per_group() -> Query<(), (Nullable<Int8>, Int8)> {
    placeholder()
}

pub fn having() -> Query<(), (Nullable<Int8>, Int8)> {
    placeholder()
}

pub fn rollup() -> Query<(), (Nullable<Int8>, Nullable<Bool>, Int8)> {
    placeholder()
}

pub fn cube() -> Query<(), (Nullable<Int8>, Nullable<Bool>, Int8)> {
    placeholder()
}

pub fn grouping_sets() -> Query<(), (Nullable<Int8>, Nullable<Bool>, Int8, Int4)> {
    placeholder()
}

pub fn common_aggregates() -> Query<
    (),
    (
        Nullable<Int8>,
        Nullable<Numeric>,
        Nullable<Text>,
        Nullable<Text>,
        Nullable<Bool>,
    ),
> {
    placeholder()
}

pub fn filter_clause() -> Query<(), (Int8, Nullable<Int8>)> {
    placeholder()
}

pub fn distinct_inside() -> Query<(), (Int8, Nullable<Array<Int4>>)> {
    placeholder()
}

pub fn order_by_inside() -> Query<(), (Nullable<Text>, Nullable<Array<Int4>>)> {
    placeholder()
}

pub fn ordered_set() -> Query<(), (Nullable<Float8>, Nullable<Int4>, Nullable<Int4>)> {
    placeholder()
}

pub fn hypothetical_set(_price_cents: i32) -> Query<(), (Int8, Int8, Float8, Float8)> {
    placeholder()
}
