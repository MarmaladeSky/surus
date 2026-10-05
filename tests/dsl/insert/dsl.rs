use crate::support::Query;

pub fn user_returning(_name: &str, _email: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn product(_name: &str, _price_cents: i32, _quantity: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn insert_select(_pattern: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn multi_row_values(_first: &str, _email: &str, _second: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn default_value(_name: &str, _price_cents: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn overriding_system_value(_id: i64, _name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn on_conflict_do_nothing(_taken: &str, _free: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn on_conflict_do_update(_name: &str, _theme: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn default_values() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn virtual_generated_column(_name: &str, _price_cents: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn on_conflict_partial_index(_name: &str, _email: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn returning_expression(_name: &str, _price_cents: i32, _quantity: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn insert_select_expression() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn insert_bound(_name: &str, _email: Option<&str>) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
