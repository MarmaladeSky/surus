use crate::support::Query;

pub fn constructor() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn comparison(_price_cents: i32, _quantity: i32) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn composite_field_access() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn composite_constructor_cast() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn composite_expansion() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
