use crate::support::Query;

pub fn set_email(_name: &str, _email: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn update_from() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn set_row_from_subquery(_user: &str, _group: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn set_row_constructor(_user: &str, _renamed: &str, _email: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn returning_star(_name: &str) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn returning_old_and_new(_name: &str, _delta: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
