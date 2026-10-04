use crate::support::Query;

pub fn typed_parameter_list(_name: &str, _price_cents: i32, _id: i64, _flag: bool) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn many_parameters(_names: &[String]) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn parameter_reuse(_value: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
