use crate::support::Query;

pub fn select_from_view() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn insert_through_updatable_view(_name: &str, _group: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn check_option_violation(_name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
