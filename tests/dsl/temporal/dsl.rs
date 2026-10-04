use crate::support::Query;

pub fn non_overlapping_insert(_room: i32, _during: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn overlap_violation(_room: i32, _during: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
