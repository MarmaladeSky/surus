use crate::support::Query;

pub fn and_or_not() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn is_null() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn is_distinct_from() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn is_true() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn between(_low: i32, _high: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn in_list(_first: &str, _second: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn like(_pattern: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn ilike(_pattern: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn similar_to(_pattern: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn like_escape(_pattern: &str, _escape: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn between_symmetric(_first: i32, _second: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn overlaps() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn arithmetic_chain() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn arithmetic_nullable_column() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn arithmetic_mixed_nullability() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn negate_expression() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn compare_expressions() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn bool_mixed_nullability() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn operator_results_combined() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn json_chain_into_text_op() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn cast_of_expression() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
