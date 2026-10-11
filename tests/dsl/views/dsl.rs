use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Nullable, Text};

pub fn select_from_view() -> Query<(), (Text, Text)> {
    placeholder()
}

pub fn insert_through_updatable_view() -> Query<(Text, Text), (Text, Nullable<Text>, Bool)> {
    placeholder()
}

pub fn check_option_violation() -> Query<(Text,), (Text,)> {
    placeholder()
}
