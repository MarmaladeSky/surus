use crate::support::Query;

pub fn line_lseg() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn lseg_box() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn lseg_lseg() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn point_box() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn point_line() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn point_lseg() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
