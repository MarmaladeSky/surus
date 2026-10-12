use crate::schema::{products, type_samples};
use surus::prelude::*;
use surus::types::{Array, Int4, Nullable, Text};
use surus::{Query, any_of, array, cardinality, query};

pub fn constructor() -> Query<(), (Array<Int4>, Array<Text>)> {
    products
        .select(|p| {
            (
                array((p.price_cents, p.quantity * 100)),
                array((p.name, "fixed")),
            )
        })
        .into()
}

pub fn subscript_and_slice() -> Query<(), (Nullable<Int4>, Nullable<Array<Int4>>, Nullable<Int4>)> {
    type_samples
        .select(|s| {
            (
                s.v_int4_array.at(1),
                s.v_int4_array.slice(2, 3),
                cardinality(s.v_int4_array),
            )
        })
        .into()
}

pub fn unnest() -> Query<(), (Nullable<Text>,)> {
    type_samples
        .select(|s| surus::unnest(s.v_text_array))
        .order_by_output(|(element,)| element)
        .into()
}

pub fn any_array_parameter() -> Query<(Array<Int4>,), (Text,)> {
    query(|(prices,)| {
        products
            .filter(|p| p.price_cents.eq(any_of(prices)))
            .select(|p| p.name)
            .order_by(|p| p.name)
    })
}
