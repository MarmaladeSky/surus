use crate::support::Query;
use schema::*;
use surus::*;

pub fn check_violation(name: &str, price_cents: i32) -> Query {
    products
        .insert((products.name, products.price_cents))
        .values((name, price_cents))
        .returning(products.name)
        .compile()
        .into()
}

pub fn unique_violation(batch: &str, name: &str) -> Query {
    tickets
        .insert((tickets.batch, tickets.name))
        .values((batch, name))
        .returning(tickets.id)
        .compile()
        .into()
}

pub fn on_conflict_on_constraint(batch: &str, name: &str) -> Query {
    tickets
        .insert((tickets.batch, tickets.name))
        .values((batch, name))
        .on_conflict_on_constraint(tickets_batch_name_key)
        .do_update(tickets.name.set(tickets.name.concat("!")))
        .returning((tickets.batch, tickets.name))
        .compile()
        .into()
}
