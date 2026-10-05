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

pub fn limit_offset_bound(_limit: i64, _offset: i64) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn fetch_first_bound(_count: i64) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn generate_series_bound(_from: i32, _to: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn tablesample_bound(_percent: f32, _seed: f64) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn jsonpath_bound(_path: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn like_pattern_bound(_pattern: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn any_array_bound(_ids: &[i64]) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn parameter_reuse_single_slot(_value: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn hostile_text(_name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
