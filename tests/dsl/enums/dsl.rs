use crate::support::Query;

pub fn declaration_order() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn comparison_with_parameter(_mood: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn enum_functions() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
