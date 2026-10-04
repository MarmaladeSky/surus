use crate::support::Query;

pub fn users_with_group() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn group_ancestor_30_levels() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn logins_30_day_pivot() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn left_join() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn left_join_null_propagation() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn right_join() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn full_join() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn cross_join() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn natural_join() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn join_using() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn lateral_first_user_per_group() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn subquery_in_from() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn tablesample_repeatable() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn with_ordinality() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn set_returning_function_in_from(_from: i32, _to: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn rows_from_multiple_functions() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
