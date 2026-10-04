use crate::support::Query;

pub fn box_box() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn point_point() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
