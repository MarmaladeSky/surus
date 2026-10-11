use crate::support::placeholder;
use surus::Query;
use surus::types::{Int4, Range, Timestamp};

pub fn non_overlapping_insert(_during: &str) -> Query<(Int4,), (Int4, Range<Timestamp>)> {
    placeholder()
}

pub fn overlap_violation(_during: &str) -> Query<(Int4,), (Int4,)> {
    placeholder()
}
