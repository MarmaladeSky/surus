use crate::support::Query;

pub fn double_colon() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn cast_function() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn invalid_text_fails_at_runtime() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
