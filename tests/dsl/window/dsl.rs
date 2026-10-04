use crate::support::Query;

pub fn row_number_by_name() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn partition_by() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn lag_lead() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn frame_clauses() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn named_window() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn value_functions() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn frame_exclusion_and_offsets() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
