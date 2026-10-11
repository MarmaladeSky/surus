use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Float8, Int4, Int8, Numeric, Text, Varchar};

pub fn double_colon() -> Query<(), (Text, Numeric, Bool)> {
    placeholder()
}

pub fn cast_function() -> Query<(), (Int8, Varchar, Float8)> {
    placeholder()
}

pub fn invalid_text_fails_at_runtime() -> Query<(), (Int4,)> {
    placeholder()
}
