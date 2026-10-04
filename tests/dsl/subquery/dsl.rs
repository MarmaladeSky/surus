use crate::support::Query;

pub fn scalar_group_name() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn scalar_multiple_rows_fails() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn exists_groups_with_users() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn not_exists_empty_groups() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn in_subquery(_group: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn any_not_the_cheapest() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn all_the_most_expensive() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn not_in_subquery(_group: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn not_in_with_null_returns_nothing() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn row_valued_in(_group: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
