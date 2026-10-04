use crate::support::Query;

pub fn count_per_group() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn having() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn rollup() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn cube() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn grouping_sets() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn common_aggregates() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn filter_clause() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn distinct_inside() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn order_by_inside() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn ordered_set() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn hypothetical_set(_price_cents: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
