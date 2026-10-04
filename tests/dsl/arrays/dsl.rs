use crate::support::Query;

pub fn constructor() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn subscript_and_slice() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn unnest() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn any_array_parameter(_prices: &[i32]) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
