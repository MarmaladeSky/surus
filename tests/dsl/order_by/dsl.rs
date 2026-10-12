use crate::schema::users;
use surus::Query;
use surus::prelude::*;
use surus::types::{Int8, Nullable, Text};

pub fn desc() -> Query<(), (Text,)> {
    users.select(|u| u.name).order_by(|u| u.name.desc()).into()
}

pub fn nulls_first() -> Query<(), (Text, Nullable<Text>)> {
    users
        .select(|u| (u.name, u.email))
        .order_by(|u| (u.email.nulls_first(), u.name))
        .into()
}

pub fn desc_nulls_last() -> Query<(), (Text, Nullable<Text>)> {
    users
        .select(|u| (u.name, u.email))
        .order_by(|u| (u.email.desc().nulls_last(), u.name))
        .into()
}

pub fn multiple_keys() -> Query<(), (Nullable<Int8>, Text)> {
    users
        .select(|u| (u.group_id, u.name))
        .order_by(|u| (u.group_id.desc(), u.name.asc()))
        .into()
}

pub fn by_expression() -> Query<(), (Text, Nullable<Text>)> {
    users
        .select(|u| (u.name, u.email))
        .order_by(|u| (u.email.is_null(), u.name))
        .into()
}

pub fn by_position() -> Query<(), (Nullable<Int8>, Text)> {
    users
        .select(|u| (u.group_id, u.name))
        .order_by(|u| (u.group_id, u.name))
        .into()
}

pub fn collate_c() -> Query<(), (Text,)> {
    users
        .select(|u| u.name)
        .order_by(|u| u.name.collate("C"))
        .into()
}

pub fn using_operator() -> Query<(), (Nullable<Int8>, Text)> {
    users
        .select(|u| (u.group_id, u.name))
        .order_by(|u| (u.group_id.desc(), u.name.asc()))
        .into()
}
