use crate::support::Query;

pub fn upsert_from_values(
    _existing: &str,
    _existing_price: i32,
    _fresh: &str,
    _fresh_price: i32,
) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn delete_not_matched_by_source(_keep: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn returning_old_and_new(_existing: &str, _fresh: &str, _price: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
