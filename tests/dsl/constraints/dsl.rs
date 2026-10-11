use crate::support::placeholder;
use surus::Query;
use surus::types::{Int8, Text};

pub fn check_violation(_price_cents: i32) -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn unique_violation() -> Query<(Text, Text), (Int8,)> {
    placeholder()
}

pub fn on_conflict_on_constraint() -> Query<(Text, Text), (Text, Text)> {
    placeholder()
}
