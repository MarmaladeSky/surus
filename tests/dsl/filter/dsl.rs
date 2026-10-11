use crate::support::placeholder;
use surus::Query;
use surus::types::{Int8, Text};

pub fn by_name() -> Query<(Text,), (Int8, Text)> {
    placeholder()
}
