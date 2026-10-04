use crate::support::Query;

pub fn quoted_table_and_columns(_user: &str, _selected: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn quoted_aliases() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn schema_qualified_tables() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
