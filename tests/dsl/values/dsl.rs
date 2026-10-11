use crate::support::placeholder;
use surus::Query;
use surus::types::{Int4, Text};

pub fn standalone() -> Query<(Int4, Text), (Int4, Text)> {
    placeholder()
}

pub fn in_from_joined_with_table() -> Query<(Text, Text), (Text, Text)> {
    placeholder()
}
