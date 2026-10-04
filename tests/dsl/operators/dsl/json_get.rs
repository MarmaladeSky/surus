use crate::support::Query;

pub fn json_int4() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn json_text() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn jsonb_int4() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn jsonb_text() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
