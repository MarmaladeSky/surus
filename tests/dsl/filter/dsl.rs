use crate::support::Query;
use schema::*;
use surus::*;

pub fn by_name(name: &str) -> Query {
    users
        .select((users.id, users.name))
        .filter(users.name.eq(name))
        .compile()
        .into()
}
