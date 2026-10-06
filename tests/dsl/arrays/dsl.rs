use crate::support::Query;
use schema::*;
use surus::*;

pub fn constructor() -> Query {
    products
        .select((
            array((products.price_cents, products.quantity * 100)),
            array((products.name, "fixed")),
        ))
        .compile()
        .into()
}

pub fn subscript_and_slice() -> Query {
    let numbers = type_samples.v_int4_array;
    type_samples
        .select((numbers.at(1), numbers.slice(2, 3), cardinality(numbers)))
        .compile()
        .into()
}

pub fn unnest() -> Query {
    let items = surus::unnest(type_samples.v_text_array);
    let (item,) = items.columns();
    type_samples
        .cross_join(items)
        .select(item)
        .order_by(item)
        .compile()
        .into()
}

pub fn any_array_parameter(prices: &[i32]) -> Query {
    products
        .select(products.name)
        .filter(products.price_cents.eq(any(prices)))
        .order_by(products.name)
        .compile()
        .into()
}
