use crate::support::Query;

pub fn tsquery() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
