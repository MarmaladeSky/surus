use crate::support::Query;

pub fn jsonb_jsonpath() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
