use crate::support::Query;

pub fn case_searched() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn case_simple() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn coalesce() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn nullif() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn greatest_least() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn case_into_arithmetic() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
