use crate::support::Query;

pub fn lseg() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn path() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
