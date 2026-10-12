use crate::schema::{groups, users};
use surus::prelude::*;
use surus::types::{Int8, Nullable, Text};
use surus::{Query, count_star, cte, from};

pub fn user_and_group_names() -> Query<(), (Text,)> {
    users
        .select(|u| u.name)
        .union(groups.select(|g| g.name))
        .order_by(|(name,)| name)
        .into()
}

pub fn union_all_keeps_duplicates() -> Query<(), (Text,)> {
    users
        .select(|u| u.name)
        .union_all(groups.select(|g| g.name))
        .order_by(|(name,)| name)
        .into()
}

pub fn intersect() -> Query<(), (Text,)> {
    users
        .select(|u| u.name)
        .intersect(groups.select(|g| g.name))
        .order_by(|(name,)| name)
        .into()
}

pub fn except() -> Query<(), (Text,)> {
    users
        .select(|u| u.name)
        .except(groups.select(|g| g.name))
        .order_by(|(name,)| name)
        .into()
}

pub fn parenthesized_with_limits() -> Query<(), (Text,)> {
    let first_user = users.select(|u| u.name).order_by(|u| u.name).limit(1);
    let last_group = groups
        .select(|g| g.name)
        .order_by(|g| g.name.desc())
        .limit(1);
    first_user.union(last_group).order_by(|(name,)| name).into()
}

pub fn union_nullable_with_not_null() -> Query<(), (Nullable<Text>,)> {
    users
        .select(|u| u.name)
        .union(users.filter(|u| u.email.is_not_null()).select(|u| u.email))
        .order_by(|(name,)| name)
        .into()
}

pub fn set_op_in_from() -> Query<(), (Int8,)> {
    let names = users.select(|u| u.name).union(groups.select(|g| g.name));
    from(names).select(|_| count_star()).into()
}

pub fn set_op_as_cte() -> Query<(), (Text,)> {
    let names = cte(users.select(|u| u.name).except(groups.select(|g| g.name)));
    from(names)
        .select(|(name,)| name.concat("!"))
        .order_by_output(|(name,)| name)
        .into()
}

pub fn nested_precedence() -> Query<(), (Nullable<Text>,)> {
    let shared = groups
        .select(|g| g.name)
        .intersect(users.select(|u| u.email));
    users
        .select(|u| u.name)
        .union(shared)
        .order_by(|(name,)| name)
        .into()
}
