use crate::schema::products;
use surus::prelude::*;
use surus::types::{Bool, Float8, Int4, Int8, Numeric, Text, Varchar};
use surus::{Query, varchar};

pub fn double_colon() -> Query<(), (Text, Numeric, Bool)> {
    products
        .select(|p| {
            (
                p.price_cents.cast::<Text>(),
                p.price_cents.cast::<Numeric>() / 100,
                p.quantity.cast::<Bool>(),
            )
        })
        .order_by(|p| p.name)
        .into()
}

pub fn cast_function() -> Query<(), (Int8, Varchar, Float8)> {
    products
        .select(|p| {
            (
                p.price_cents.cast::<Int8>(),
                p.name.cast_to(varchar(5)),
                p.quantity.cast::<Float8>(),
            )
        })
        .order_by(|p| p.name)
        .into()
}

pub fn invalid_text_fails_at_runtime() -> Query<(), (Int4,)> {
    products.select(|p| p.name.cast::<Int4>()).into()
}
