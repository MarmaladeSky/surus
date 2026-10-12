use crate::schema::{groups, products, users};
use surus::prelude::*;
use surus::types::{Bool, Int4, Int8, Nullable, Text};
use surus::{ColumnList, Query, query, row};

pub fn set_email() -> Query<(Text, Text), (Text, Nullable<Text>)> {
    query(|(email, name)| {
        users
            .filter(|u| u.name.eq(name))
            .update(|u| u.email.set(email))
            .returning(|u| (u.name, u.email))
    })
}

pub fn update_from() -> Query<(), (Text, Nullable<Text>)> {
    users
        .join(groups, |(u, g)| g.id.eq(u.group_id))
        .update(|(u, g)| u.email.set(g.name.concat("@example.com")))
        .returning(|(u, _)| (u.name, u.email))
        .into()
}

pub fn set_row_from_subquery() -> Query<(Text, Text), (Text, Nullable<Text>, Bool)> {
    query(|(group, name)| {
        users
            .filter(|u| u.name.eq(name))
            .update(|u| {
                let group = groups
                    .filter(|g| g.name.eq(group))
                    .select(|g| (g.name.concat("@example.com"), g.id));
                (u.email, u.group_id).set(group)
            })
            .returning(|u| (u.name, u.email, u.group_id.is_not_null()))
    })
}

pub fn set_row_constructor() -> Query<(Text, Text, Text), (Text, Nullable<Text>)> {
    query(|(renamed, email, name)| {
        users
            .filter(|u| u.name.eq(name))
            .update(|u| (u.name, u.email).set(row((renamed, email))))
            .returning(|u| (u.name, u.email))
    })
}

pub fn returning_star() -> Query<(Text,), (Int8, Text, Int4, Int4, Nullable<Int8>, Nullable<Int8>)>
{
    query(|(name,)| {
        products
            .filter(|p| p.name.eq(name))
            .update(|p| p.quantity.set(p.quantity + 1))
            .returning(|p| p.star())
    })
}

pub fn returning_old_and_new(delta: i32) -> Query<(Text,), (Int4, Int4, Nullable<Int8>)> {
    query(|(name,)| {
        products
            .filter(|p| p.name.eq(name))
            .update(|p| p.quantity.set(p.quantity + delta))
            .returning_old_new(|(old, new)| {
                (
                    old.quantity,
                    new.quantity,
                    new.total_cents - old.total_cents,
                )
            })
    })
}

pub fn update_set_expression() -> Query<(), (Text, Int4)> {
    products
        .filter(|p| p.price_cents.gt(0))
        .update(|p| p.quantity.set(p.quantity * 2 + 1))
        .returning(|p| (p.name, p.quantity))
        .into()
}

pub fn update_from_join_predicate() -> Query<(), (Text, Nullable<Text>)> {
    users
        .join(groups, |(u, g)| g.id.eq(u.group_id))
        .filter(|(u, _)| u.email.is_null())
        .update(|(u, g)| u.email.set(g.name.concat("@x")))
        .returning(|(u, _)| (u.name, u.email))
        .into()
}

pub fn update_bound() -> Query<(Text, Int8), (Int8, Text)> {
    query(|(name, id)| {
        users
            .filter(|u| u.id.eq(id))
            .update(|u| u.name.set(name))
            .returning(|u| (u.id, u.name))
    })
}
