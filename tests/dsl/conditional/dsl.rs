use crate::schema::{products, users};
use surus::prelude::*;
use surus::types::{Int4, Nullable, Text};
use surus::{Query, case, greatest, least, when};

pub fn case_searched() -> Query<(), (Text, Text)> {
    users
        .select(|u| {
            let status = when(u.email.is_null(), "no email")
                .when(u.group_id.is_null(), "no group")
                .otherwise("complete");
            (u.name, status)
        })
        .order_by(|u| u.name)
        .into()
}

pub fn case_simple() -> Query<(), (Text, Text)> {
    products
        .select(|p| {
            let stock = case(p.quantity)
                .when(0, "out")
                .when(1, "last")
                .otherwise("in stock");
            (p.name, stock)
        })
        .order_by(|p| p.name)
        .into()
}

pub fn coalesce() -> Query<(), (Text, Text)> {
    users
        .select(|u| (u.name, surus::coalesce(u.email, u.name)))
        .order_by(|u| u.name)
        .into()
}

pub fn nullif() -> Query<(), (Text, Nullable<Int4>)> {
    products
        .select(|p| (p.name, surus::nullif(p.quantity, 0)))
        .order_by(|p| p.name)
        .into()
}

pub fn greatest_least() -> Query<(), (Text, Int4, Int4)> {
    products
        .select(|p| {
            (
                p.name,
                greatest(p.price_cents, p.quantity * 100),
                least(p.price_cents, p.quantity * 100),
            )
        })
        .order_by(|p| p.name)
        .into()
}

pub fn case_into_arithmetic() -> Query<(), (Text, Int4)> {
    products
        .select(|p| {
            let price = when(p.quantity.gt(0), p.price_cents).otherwise(0);
            (p.name, price + 1)
        })
        .order_by(|p| p.name)
        .into()
}
