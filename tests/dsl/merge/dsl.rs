use crate::support::placeholder;
use surus::Query;
use surus::types::{Int4, Nullable, Text};

pub fn upsert_from_values() -> Query<(Text, Int4, Text, Int4), (Text, Text, Int4, Int4)> {
    placeholder()
}

pub fn delete_not_matched_by_source() -> Query<(Text,), (Text, Text)> {
    placeholder()
}

pub fn returning_old_and_new() -> Query<(Text, Int4, Text), (Text, Nullable<Int4>, Int4, Text)> {
    placeholder()
}
