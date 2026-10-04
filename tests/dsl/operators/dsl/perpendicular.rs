use crate::support::Query;

pub fn line_line() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn lseg_lseg() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
