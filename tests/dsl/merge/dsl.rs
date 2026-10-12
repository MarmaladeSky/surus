use crate::schema::products;
use surus::prelude::*;
use surus::types::{Int4, Nullable, Text};
use surus::{Query, merge_action, query, values};

pub fn upsert_from_values() -> Query<(Text, Int4, Text, Int4), (Text, Text, Int4, Int4)> {
    query(|(existing, existing_price, fresh, fresh_price)| {
        let source = values((existing, existing_price, 1)).row((fresh, fresh_price, 2));
        products
            .merge(source, |(p, (name, ..))| p.name.eq(name))
            .when_matched_update(|(p, (_, price_cents, _))| p.price_cents.set(price_cents))
            .when_not_matched_insert(|row, (name, price_cents, quantity)| {
                row.name(name).price_cents(price_cents).quantity(quantity)
            })
            .returning(|(p, _)| (merge_action(), p.name, p.price_cents, p.quantity))
    })
}

pub fn delete_not_matched_by_source() -> Query<(Text,), (Text, Text)> {
    query(|(keep,)| {
        products
            .merge(values((keep,)), |(p, (name,))| p.name.eq(name))
            .when_matched_do_nothing()
            .when_not_matched_by_source_delete()
            .returning(|(p, _)| (merge_action(), p.name))
    })
}

pub fn returning_old_and_new() -> Query<(Text, Int4, Text), (Text, Nullable<Int4>, Int4, Text)> {
    query(|(existing, price_cents, fresh)| {
        let source = values((existing, price_cents)).row((fresh, price_cents));
        products
            .merge(source, |(p, (name, _))| p.name.eq(name))
            .when_matched_update(|(p, (_, price_cents))| p.price_cents.set(price_cents))
            .when_not_matched_insert(|row, (name, price_cents)| {
                row.name(name).price_cents(price_cents)
            })
            .returning_old_new(|(old, new, _)| {
                (merge_action(), old.price_cents, new.price_cents, new.name)
            })
    })
}
