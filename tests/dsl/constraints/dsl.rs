use crate::schema::{products, tickets, tickets_batch_name_key};
use surus::prelude::*;
use surus::types::{Int8, Text};
use surus::{Query, query};

pub fn check_violation(price_cents: i32) -> Query<(Text,), (Text,)> {
    query(|(name,)| {
        products
            .insert(|p| p.name(name).price_cents(price_cents))
            .returning(|p| p.name)
    })
}

pub fn unique_violation() -> Query<(Text, Text), (Int8,)> {
    query(|(batch, name)| {
        tickets
            .insert(|t| t.batch(batch).name(name))
            .returning(|t| t.id)
    })
}

pub fn on_conflict_on_constraint() -> Query<(Text, Text), (Text, Text)> {
    query(|(batch, name)| {
        tickets
            .insert(|t| t.batch(batch).name(name))
            .on_constraint(tickets_batch_name_key)
            .do_update(|(t, _)| t.name.set(t.name.concat("!")))
            .returning(|t| (t.batch, t.name))
    })
}
