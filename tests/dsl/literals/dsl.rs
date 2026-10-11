use crate::support::placeholder;
use surus::Query;
use surus::types::{Bit, Bool, Int4, Interval, Nullable, Numeric, Text, Time, Timestamp};

pub fn quotes_and_backslashes(_name: &str) -> Query<(), (Text, Int4)> {
    placeholder()
}

pub fn non_ascii(_name: &str) -> Query<(), (Text, Int4, Text)> {
    placeholder()
}

pub fn typed_literals() -> Query<
    (),
    (
        Nullable<Timestamp>,
        Nullable<Bool>,
        Nullable<Interval>,
        Time,
        Numeric,
    ),
> {
    placeholder()
}

pub fn escape_and_dollar_quoted_strings() -> Query<(), (Text,)> {
    placeholder()
}

pub fn bit_and_hex_literals() -> Query<(), (Nullable<Bit>, Nullable<Bool>, Int4, Int4)> {
    placeholder()
}
