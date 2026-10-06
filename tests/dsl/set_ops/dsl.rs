use crate::support::Query;
use schema::*;
use surus::*;

pub fn user_and_group_names() -> Query {
    users
        .select(users.name)
        .union(groups.select(groups.name))
        .order_by(|(name,)| name)
        .compile()
        .into()
}

pub fn union_all_keeps_duplicates() -> Query {
    users
        .select(users.name)
        .union_all(groups.select(groups.name))
        .order_by(|(name,)| name)
        .compile()
        .into()
}

pub fn intersect() -> Query {
    users
        .select(users.name)
        .intersect(groups.select(groups.name))
        .order_by(|(name,)| name)
        .compile()
        .into()
}

pub fn except() -> Query {
    users
        .select(users.name)
        .except(groups.select(groups.name))
        .order_by(|(name,)| name)
        .compile()
        .into()
}

pub fn parenthesized_with_limits() -> Query {
    let first_user = users.select(users.name).order_by(users.name).limit(1);
    let last_group = groups
        .select(groups.name)
        .order_by(groups.name.desc())
        .limit(1);
    first_user
        .union(last_group)
        .order_by(|(name,)| name)
        .compile()
        .into()
}

pub fn union_nullable_with_not_null() -> Query {
    let emails = users.select(users.email).filter(users.email.is_not_null());
    users
        .select(users.name)
        .union(emails)
        .order_by(|(name,)| name)
        .compile()
        .into()
}

pub fn set_op_in_from() -> Query {
    let s = alias!(
        users.select(users.name).union(groups.select(groups.name)),
        "s"
    );
    s.select(count_star()).compile().into()
}

pub fn set_op_as_cte() -> Query {
    let names = cte(
        "names",
        users.select(users.name).except(groups.select(groups.name)),
    );
    let (name,) = names.columns();
    names
        .select(name.concat("!"))
        .order_by(name.concat("!"))
        .compile()
        .into()
}

pub fn nested_precedence() -> Query {
    let shared = groups
        .select(groups.name)
        .intersect(users.select(users.email));
    users
        .select(users.name)
        .union(shared)
        .order_by(|(name,)| name)
        .compile()
        .into()
}
