use crate::support::Query;

pub fn at_time_zone(_utc: &str, _berlin: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
