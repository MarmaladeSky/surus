use crate::schema::{groups, products, users};
use surus::prelude::*;
use surus::types::{Int8, Nullable, Text};
use surus::{Query, all, any, avg, count_star, exists, query, row};

pub fn scalar_group_name() -> Query<(), (Text, Nullable<Text>)> {
    users
        .select(|u| {
            let group = groups.filter(|g| g.id.eq(u.group_id)).select(|g| g.name);
            (u.name, group)
        })
        .order_by(|u| u.name)
        .into()
}

pub fn scalar_multiple_rows_fails() -> Query<(), (Text, Nullable<Text>)> {
    users.select(|u| (u.name, groups.select(|g| g.name))).into()
}

pub fn exists_groups_with_users() -> Query<(), (Text,)> {
    groups
        .filter(|g| exists(users.filter(|u| u.group_id.eq(g.id)).select(|_| 1)))
        .select(|g| g.name)
        .order_by(|g| g.name)
        .into()
}

pub fn not_exists_empty_groups() -> Query<(), (Text,)> {
    groups
        .filter(|g| exists(users.filter(|u| u.group_id.eq(g.id)).select(|_| 1)).not())
        .select(|g| g.name)
        .order_by(|g| g.name)
        .into()
}

pub fn in_subquery() -> Query<(Text,), (Text,)> {
    query(|(name,)| {
        users
            .filter(|u| {
                u.group_id
                    .is_in(groups.filter(|g| g.name.eq(name)).select(|g| g.id))
            })
            .select(|u| u.name)
            .order_by(|u| u.name)
    })
}

pub fn any_not_the_cheapest() -> Query<(), (Text,)> {
    products
        .filter(|p| p.price_cents.gt(any(products.select(|p| p.price_cents))))
        .select(|p| p.name)
        .order_by(|p| p.name)
        .into()
}

pub fn all_the_most_expensive() -> Query<(), (Text,)> {
    products
        .filter(|p| p.price_cents.ge(all(products.select(|p| p.price_cents))))
        .select(|p| p.name)
        .order_by(|p| p.name)
        .into()
}

pub fn not_in_subquery() -> Query<(Text,), (Text,)> {
    query(|(name,)| {
        users
            .filter(|u| {
                u.group_id
                    .is_not_in(groups.filter(|g| g.name.eq(name)).select(|g| g.id))
            })
            .select(|u| u.name)
            .order_by(|u| u.name)
    })
}

pub fn not_in_with_null_returns_nothing() -> Query<(), (Text, Int8)> {
    let with_null = users
        .filter(|u| u.group_id.is_not_in(users.select(|u| u.group_id)))
        .select(|_| ("with null", count_star()));
    let without_null = users
        .filter(|u| {
            let known = users
                .filter(|u| u.group_id.is_not_null())
                .select(|u| u.group_id);
            u.group_id.is_not_in(known)
        })
        .select(|_| ("without null", count_star()));
    with_null.union_all(without_null).into()
}

pub fn row_valued_in() -> Query<(Text,), (Text,)> {
    query(|(name,)| {
        users
            .filter(|u| row((u.group_id, true)).is_in(groups.select(|g| (g.id, g.name.eq(name)))))
            .select(|u| u.name)
            .order_by(|u| u.name)
    })
}

pub fn correlated_scalar_in_arithmetic() -> Query<(), (Text, Nullable<Int8>)> {
    groups
        .select(|g| {
            let members = users
                .filter(|u| u.group_id.eq(g.id))
                .select(|_| count_star());
            (g.name, surus::value(members) + 1)
        })
        .order_by(|g| g.name)
        .into()
}

pub fn exists_and_predicate() -> Query<(), (Text,)> {
    groups
        .filter(|g| {
            exists(users.filter(|u| u.group_id.eq(g.id)).select(|_| 1)).and(g.parent_id.is_null())
        })
        .select(|g| g.name)
        .order_by(|g| g.name)
        .into()
}

pub fn scalar_subquery_compared() -> Query<(), (Text,)> {
    products
        .filter(|p| p.price_cents.gt(products.select(|p| avg(p.price_cents))))
        .select(|p| p.name)
        .order_by(|p| p.name)
        .into()
}
