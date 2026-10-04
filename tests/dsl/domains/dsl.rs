use crate::support::Query;

pub fn cast_to_domain() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn check_violation_on_cast() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn check_violation_on_insert(_value: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
