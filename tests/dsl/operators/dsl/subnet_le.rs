use crate::support::Query;

pub fn inet_inet() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
