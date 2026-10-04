use crate::support::Query;

pub fn quotes_and_backslashes(_name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn non_ascii(_name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn typed_literals() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn escape_and_dollar_quoted_strings() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn bit_and_hex_literals() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
