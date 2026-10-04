use crate::support::Query;

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

pub fn tsrange_tsrange() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
