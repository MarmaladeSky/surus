use crate::schema::{groups, tickets, users};
use surus::prelude::*;
use surus::types::{Bool, Int4, Int8, Nullable, Text};
use surus::{Query, current_date, query, select, value};

pub fn single_row() -> Query<(), (Int8, Text, Nullable<Text>, Nullable<Int8>)> {
    users.select(|u| (u.id, u.name, u.email, u.group_id)).into()
}

pub fn distinct_group_ids() -> Query<(), (Nullable<Int8>,)> {
    users
        .select(|u| u.group_id)
        .distinct()
        .order_by(|u| u.group_id)
        .into()
}

pub fn distinct_on_first_user_per_group() -> Query<(), (Nullable<Int8>, Text)> {
    users
        .select(|u| (u.group_id, u.name))
        .distinct_on(|u| u.group_id)
        .order_by(|u| (u.group_id, u.name))
        .into()
}

pub fn limit_offset(limit: i64, offset: i64) -> Query<(), (Text,)> {
    users
        .select(|u| u.name)
        .order_by(|u| u.name)
        .limit(limit)
        .offset(offset)
        .into()
}

pub fn fetch_first_rows_only(count: i64, offset: i64) -> Query<(), (Text,)> {
    users
        .select(|u| u.name)
        .order_by(|u| u.name)
        .offset(offset)
        .fetch_first(count)
        .into()
}

pub fn fetch_first_with_ties(count: i64) -> Query<(), (Nullable<Int8>,)> {
    users
        .select(|u| u.group_id)
        .order_by(|u| u.group_id)
        .fetch_first_with_ties(count)
        .into()
}

pub fn aliases() -> Query<(), (Text, Text)> {
    users
        .join(groups, |(u, g)| g.id.eq(u.group_id))
        .select(|(u, g)| (u.name.alias("user_name"), g.name.alias("group_name")))
        .order_by(|(u, _)| u.name)
        .into()
}

pub fn for_update() -> Query<(Text,), (Text,)> {
    query(|(batch,)| {
        tickets
            .filter(|t| t.batch.eq(batch))
            .select(|t| t.name)
            .order_by(|t| t.name)
            .for_update()
    })
}

pub fn for_share() -> Query<(Text,), (Text,)> {
    query(|(batch,)| {
        tickets
            .filter(|t| t.batch.eq(batch))
            .select(|t| t.name)
            .order_by(|t| t.name)
            .for_share()
    })
}

pub fn for_update_skip_locked() -> Query<(Text,), (Text,)> {
    query(|(batch,)| {
        tickets
            .filter(|t| t.batch.eq(batch))
            .select(|t| t.name)
            .order_by(|t| t.name)
            .for_update()
            .skip_locked()
    })
}

pub fn for_update_nowait() -> Query<(Text,), (Text,)> {
    query(|(name,)| {
        tickets
            .filter(|t| t.name.eq(name))
            .select(|t| t.name)
            .for_update()
            .nowait()
    })
}

pub fn star() -> Query<(), (Int8, Text, Nullable<Text>, Nullable<Int8>)> {
    users.select(|u| u.star()).order_by(|u| u.name).into()
}

pub fn table_star() -> Query<(), (Int8, Text, Nullable<Text>, Nullable<Int8>, Text)> {
    users
        .join(groups, |(u, g)| g.id.eq(u.group_id))
        .select(|(u, g)| (u.star(), g.name.alias("group_name")))
        .order_by(|(u, _)| u.name)
        .into()
}

pub fn without_from() -> Query<(), (Int4, Text, Bool, Bool)> {
    select((
        value(1) + 1,
        value("a").concat("b"),
        value(10).gt(3),
        current_date().eq(current_date()),
    ))
    .into()
}

pub fn for_key_share() -> Query<(Text,), (Text,)> {
    query(|(batch,)| {
        tickets
            .filter(|t| t.batch.eq(batch))
            .select(|t| t.name)
            .order_by(|t| t.name)
            .for_key_share()
    })
}

pub fn for_no_key_update_skip_locked() -> Query<(Text,), (Text,)> {
    query(|(batch,)| {
        tickets
            .filter(|t| t.batch.eq(batch))
            .select(|t| t.name)
            .order_by(|t| t.name)
            .for_no_key_update()
            .skip_locked()
    })
}

pub fn for_update_of_table() -> Query<(), (Text, Text)> {
    users
        .join(groups, |(u, g)| g.id.eq(u.group_id))
        .select(|(u, g)| (u.name, g.name))
        .order_by(|(u, _)| u.name)
        .for_update()
        .of(|(u, _)| u)
        .into()
}
