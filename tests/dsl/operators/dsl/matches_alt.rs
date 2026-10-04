use crate::support::Query;

pub fn tsquery_tsvector() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn tsvector_tsquery() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
