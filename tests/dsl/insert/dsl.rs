use crate::schema::{groups, products, type_samples, user_settings, users};
use surus::prelude::*;
use surus::types::{Bool, Int4, Int8, Nullable, Text};
use surus::{Query, default, from, null, query};

pub fn user_returning() -> Query<(Text, Text), (Text, Nullable<Text>)> {
    query(|(name, email)| {
        users
            .insert(|u| u.name(name).email(email))
            .returning(|u| (u.name, u.email))
    })
}

pub fn product() -> Query<(Text, Int4, Int4), (Text, Int4, Int4, Nullable<Int8>)> {
    query(|(name, price_cents, quantity)| {
        products
            .insert(|p| p.name(name).price_cents(price_cents).quantity(quantity))
            .returning(|p| (p.name, p.price_cents, p.quantity, p.total_cents))
    })
}

pub fn insert_select() -> Query<(Text,), (Text,)> {
    query(|(pattern,)| {
        let matching = users.filter(|u| u.name.like(pattern)).order_by(|u| u.name);
        groups
            .insert_from(matching, |g, u| g.name(u.name))
            .returning(|g| g.name)
    })
}

pub fn multi_row_values() -> Query<(Text, Text, Text), (Text, Nullable<Text>)> {
    query(|(first, email, second)| {
        users
            .insert(|u| u.name(first).email(email))
            .values(|u| u.name(second).email(null()))
            .returning(|u| (u.name, u.email))
    })
}

pub fn default_value() -> Query<(Text, Int4), (Text, Int4, Int4)> {
    query(|(name, price_cents)| {
        products
            .insert(|p| p.name(name).price_cents(price_cents).quantity(default()))
            .returning(|p| (p.name, p.price_cents, p.quantity))
    })
}

pub fn overriding_system_value() -> Query<(Int8, Text), (Int8, Text)> {
    query(|(id, name)| {
        products
            .insert(|p| p.id(id).name(name).price_cents(1))
            .returning(|p| (p.id, p.name))
    })
}

pub fn on_conflict_do_nothing() -> Query<(Text, Text), (Int8, Text)> {
    query(|(taken, free)| {
        let chosen = users.filter(|u| u.name.is_in((taken, free)));
        user_settings
            .insert_from(chosen, |s, u| s.user_id(u.id).theme("light"))
            .on_conflict(|s| s.user_id)
            .do_nothing()
            .returning(|s| (s.user_id, s.theme))
    })
}

pub fn on_conflict_do_update(theme: &str) -> Query<(Text,), (Int8, Text)> {
    query(|(name,)| {
        user_settings
            .insert_from(users.filter(|u| u.name.eq(name)), |s, u| {
                s.user_id(u.id).theme(theme)
            })
            .on_conflict(|s| s.user_id)
            .do_update_where(
                |(s, excluded)| s.theme.set(excluded.theme.concat("!")),
                |(s, excluded)| s.theme.ne(excluded.theme),
            )
            .returning(|s| (s.user_id, s.theme))
    })
}

pub fn default_values() -> Query<(), (Bool, Bool)> {
    type_samples
        .insert_defaults()
        .returning(|s| (s.v_int4.is_null(), s.v_text.is_null()))
        .into()
}

pub fn virtual_generated_column() -> Query<(Text, Int4), (Int4, Nullable<Int8>, Nullable<Int8>)> {
    query(|(name, price_cents)| {
        products
            .insert(|p| p.name(name).price_cents(price_cents))
            .returning(|p| (p.price_cents, p.price_with_tax, p.total_cents))
    })
}

pub fn on_conflict_partial_index() -> Query<(Text, Text), (Text, Nullable<Text>)> {
    query(|(name, email)| {
        users
            .insert(|u| u.name(name).email(email))
            .on_conflict_where(|u| u.email, |u| u.email.is_not_null())
            .do_update(|(u, excluded)| u.name.set(excluded.name))
            .returning(|u| (u.name, u.email))
    })
}

pub fn returning_expression() -> Query<(Text, Int4, Int4), (Int4,)> {
    query(|(name, price_cents, quantity)| {
        products
            .insert(|p| p.name(name).price_cents(price_cents).quantity(quantity))
            .returning(|p| (p.price_cents * p.quantity).alias("total"))
    })
}

pub fn insert_select_expression() -> Query<(), (Text,)> {
    groups
        .insert_from(from(groups), |row, g| row.name(g.name.concat("-copy")))
        .returning(|g| g.name)
        .into()
}

pub fn insert_bound() -> Query<(Text, Nullable<Text>), (Text, Nullable<Text>)> {
    query(|(name, email)| {
        users
            .insert(|u| u.name(name).email(email))
            .returning(|u| (u.name, u.email))
    })
}
