use crate::support::Query;
use schema::*;
use surus::*;

pub fn desc() -> Query {
    users
        .select(users.name)
        .order_by(users.name.desc())
        .compile()
        .into()
}

pub fn nulls_first() -> Query {
    users
        .select((users.name, users.email))
        .order_by((users.email.nulls_first(), users.name))
        .compile()
        .into()
}

pub fn desc_nulls_last() -> Query {
    users
        .select((users.name, users.email))
        .order_by((users.email.desc().nulls_last(), users.name))
        .compile()
        .into()
}

pub fn multiple_keys() -> Query {
    users
        .select((users.group_id, users.name))
        .order_by((users.group_id.desc(), users.name.asc()))
        .compile()
        .into()
}

pub fn by_expression() -> Query {
    users
        .select((users.name, users.email))
        .order_by((users.email.is_null(), users.name))
        .compile()
        .into()
}

pub fn by_position() -> Query {
    users
        .select((users.group_id, users.name))
        .order_by((users.group_id, users.name))
        .compile()
        .into()
}

pub fn collate_c() -> Query {
    users
        .select(users.name)
        .order_by(users.name.collate("C"))
        .compile()
        .into()
}

pub fn using_operator() -> Query {
    users
        .select((users.group_id, users.name))
        .order_by((users.group_id.using(op::Gt), users.name.using(op::Lt)))
        .compile()
        .into()
}
