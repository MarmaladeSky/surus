use crate::support::Query;

pub fn jsonb_text_array() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
