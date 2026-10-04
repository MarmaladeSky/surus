use crate::support::Query;

pub fn constructors_and_accessors() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn containment_and_overlap() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn multirange_functions() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn range_aggregates() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
