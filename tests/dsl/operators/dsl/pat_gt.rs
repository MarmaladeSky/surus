use crate::support::Query;

pub fn bpchar_bpchar() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn text_text() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
