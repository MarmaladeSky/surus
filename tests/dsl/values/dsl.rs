use crate::support::Query;
use schema::*;
use surus::*;

pub fn standalone(first: i32, second: &str) -> Query {
    values([(first, second), (first + 1, second)])
        .compile()
        .into()
}

pub fn in_from_joined_with_table(first: &str, second: &str) -> Query {
    let v = alias!(values([(first, "first"), (second, "second")]), "v");
    let (name, label) = v.columns();
    users
        .join(v, name.eq(users.name))
        .select((users.name, label))
        .order_by(label)
        .compile()
        .into()
}
