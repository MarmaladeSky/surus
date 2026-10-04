use crate::support::Query;

pub fn circle() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn jsonb_jsonpath() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn lseg() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn polygon() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn r#box() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn text_text() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn text_tsquery() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn tsquery_tsvector() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn tsvector_tsquery() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
