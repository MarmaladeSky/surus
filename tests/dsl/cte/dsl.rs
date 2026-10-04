use crate::support::Query;

pub fn named_users(_name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn recursive_ancestors(_leaf: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn recursive_descendants_with_path(_root: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn data_modifying(_group: &str, _batch: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn chained_materialized() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
