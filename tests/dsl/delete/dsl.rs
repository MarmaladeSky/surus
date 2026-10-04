use crate::support::Query;

pub fn by_name(_name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn delete_using(_group: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn returning_table_star(_group: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn returning_old(_name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
