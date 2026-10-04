use crate::support::Query;

pub fn bpchar_text() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn bytea_bytea() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn text_text() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
