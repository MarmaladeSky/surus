use crate::support::Query;
use schema::*;
use surus::*;

pub fn constructor() -> Query {
    products
        .select((
            row((products.name, products.price_cents)),
            row((products.quantity, products.name)),
        ))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn comparison(price_cents: i32, quantity: i32) -> Query {
    products
        .select(products.name)
        .filter(row((products.price_cents, products.quantity)).gt(row((price_cents, quantity))))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn composite_field_access() -> Query {
    let tag = type_samples.v_price_tag;
    type_samples
        .select((tag.amount(), tag.currency(), tag))
        .compile()
        .into()
}

pub fn composite_constructor_cast() -> Query {
    let price = row((products.price_cents / Decimal::new(1000, 1), "USD")).cast::<PriceTag>();
    let currency = row((Decimal::new(125, 2), "EUR"))
        .cast::<PriceTag>()
        .currency();
    products
        .select((price, currency))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn composite_expansion() -> Query {
    type_samples
        .select(expand(type_samples.v_price_tag))
        .compile()
        .into()
}
