use crate::support::Query;

pub fn desc() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn nulls_first() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn desc_nulls_last() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn multiple_keys() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn by_expression() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn by_position() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn collate_c() -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn using_operator() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
