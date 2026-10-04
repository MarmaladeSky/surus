use crate::support::Query;

pub fn check_violation(_name: &str, _price_cents: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn unique_violation(_batch: &str, _name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn on_conflict_on_constraint(_batch: &str, _name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
