use crate::support::Query;

pub fn user_and_group_names() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn union_all_keeps_duplicates() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn intersect() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn except() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn parenthesized_with_limits() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
