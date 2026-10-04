use crate::support::Query;

pub fn by_name(_name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
