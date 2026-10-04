use crate::support::Query;

pub fn float8() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
