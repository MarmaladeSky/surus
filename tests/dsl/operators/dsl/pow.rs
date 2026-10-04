use crate::support::Query;

pub fn float8_float8() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn numeric_numeric() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
