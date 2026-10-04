use crate::support::Query;

pub fn text_text() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
