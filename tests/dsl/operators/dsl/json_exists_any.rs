use crate::support::Query;

pub fn jsonb_text_array() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn line() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn lseg() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn point_point() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
