use crate::schema::{PriceTag, PriceTagFields, products, type_samples};
use surus::prelude::*;
use surus::types::{Nullable, Numeric, Record, Text};
use surus::{Query, expand, literal, row};

pub fn constructor() -> Query<(), (Record, Record)> {
    products
        .select(|p| (row((p.name, p.price_cents)), row((p.quantity, p.name))))
        .order_by(|p| p.name)
        .into()
}

pub fn comparison(price_cents: i32, quantity: i32) -> Query<(), (Text,)> {
    products
        .filter(|p| row((p.price_cents, p.quantity)).gt(row((price_cents, quantity))))
        .select(|p| p.name)
        .order_by(|p| p.name)
        .into()
}

pub fn composite_field_access() -> Query<(), (Nullable<Numeric>, Nullable<Text>, Nullable<PriceTag>)>
{
    type_samples
        .select(|s| {
            (
                s.v_price_tag.amount(),
                s.v_price_tag.currency(),
                s.v_price_tag,
            )
        })
        .into()
}

pub fn composite_constructor_cast() -> Query<(), (PriceTag, Nullable<Text>)> {
    products
        .select(|p| {
            (
                PriceTag::row(p.price_cents / literal::<Numeric>("100.0"), "USD"),
                PriceTag::row(literal::<Numeric>("1.25"), "EUR").currency(),
            )
        })
        .order_by(|p| p.name)
        .into()
}

pub fn composite_expansion() -> Query<(), (Nullable<Numeric>, Nullable<Text>)> {
    type_samples.select(|s| expand(s.v_price_tag)).into()
}
