use crate::support::Query;
use schema::*;
use surus::*;

pub fn double_colon() -> Query {
    products
        .select((
            products.price_cents.cast::<Text>(),
            products.price_cents.cast::<Numeric>() / 100,
            products.quantity.cast::<Bool>(),
        ))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn cast_function() -> Query {
    products
        .select((
            products.price_cents.cast::<Int8>(),
            products.name.cast::<Varchar<5>>(),
            products.quantity.cast::<Float8>(),
        ))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn invalid_text_fails_at_runtime() -> Query {
    products
        .select(products.name.cast::<Int4>())
        .compile()
        .into()
}
