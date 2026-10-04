use crate::support::Query;

pub fn standalone(_first: i32, _second: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn in_from_joined_with_table(_first: &str, _second: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
