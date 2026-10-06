use crate::support::Query;
use schema::*;
use surus::*;

pub fn upsert_from_values(
    existing: &str,
    existing_price: i32,
    fresh: &str,
    fresh_price: i32,
) -> Query {
    let src = alias!(
        values([(existing, existing_price, 1), (fresh, fresh_price, 2)]),
        "src"
    );
    let (name, price_cents, quantity) = src.columns();
    products
        .merge(src, products.name.eq(name))
        .when_matched(update(products.price_cents.set(price_cents)))
        .when_not_matched(insert(
            (products.name, products.price_cents, products.quantity),
            (name, price_cents, quantity),
        ))
        .returning((
            merge_action(),
            products.name,
            products.price_cents,
            products.quantity,
        ))
        .compile()
        .into()
}

pub fn delete_not_matched_by_source(keep: &str) -> Query {
    let src = alias!(values([(keep,)]), "src");
    let (name,) = src.columns();
    products
        .merge(src, products.name.eq(name))
        .when_matched(do_nothing())
        .when_not_matched_by_source(delete())
        .returning((merge_action(), products.name))
        .compile()
        .into()
}

pub fn returning_old_and_new(existing: &str, fresh: &str, price: i32) -> Query {
    let src = alias!(values([(existing, price), (fresh, price)]), "src");
    let (name, price_cents) = src.columns();
    let (old, new) = (products.old(), products.new());
    products
        .merge(src, products.name.eq(name))
        .when_matched(update(products.price_cents.set(price_cents)))
        .when_not_matched(insert(
            (products.name, products.price_cents),
            (name, price_cents),
        ))
        .returning((merge_action(), old.price_cents, new.price_cents, new.name))
        .compile()
        .into()
}
