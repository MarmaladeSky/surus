use crate::support::Query;

pub fn single_row() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn distinct_group_ids() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn distinct_on_first_user_per_group() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn limit_offset(_limit: i64, _offset: i64) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn fetch_first_rows_only(_count: i64, _offset: i64) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn fetch_first_with_ties(_count: i64) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn aliases() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn for_update(_batch: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn for_share(_batch: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn for_update_skip_locked(_batch: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn for_update_nowait(_name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn star() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn table_star() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn without_from() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn for_key_share(_batch: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn for_no_key_update_skip_locked(_batch: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn for_update_of_table() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
