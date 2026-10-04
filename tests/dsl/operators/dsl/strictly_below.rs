use crate::support::Query;

pub fn box_box() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn circle_circle() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn point_point() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn polygon_polygon() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
