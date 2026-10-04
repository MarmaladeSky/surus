use crate::support::Query;

pub fn jsonb_text() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
