use crate::support::Query;

pub fn box_box() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn circle_circle() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn inet_inet() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int4_array_int4_array() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int4multirange_int4multirange() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int4multirange_int4range() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int4range_int4multirange() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int4range_int4range() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn polygon_polygon() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn text_array_text_array() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn tsquery_tsquery() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn tsrange_tsrange() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
