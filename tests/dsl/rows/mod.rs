mod dsl;

use crate::schema::PriceTag;
use crate::support::{Case, compile, literal, rnd, sql};
use surus::Query;
use surus::types::{Nullable, Numeric, Record, Text};

fn seed_products(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity)
         VALUES ('{}', 400, 1), ('{}', 500, 1), ('{}', 500, 2), ('{}', 600, 0)",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));
}

#[test]
fn constructor() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Record, Record)> =
        sql("SELECT ROW(name, price_cents), (quantity, name) FROM products ORDER BY name");
    case.assert_same(expected, compile(dsl::constructor), ());
}

#[test]
fn comparison() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text,)> =
        sql("SELECT name FROM products WHERE (price_cents, quantity) > (500, 1) ORDER BY name");
    case.assert_same(expected, compile(|| dsl::comparison(500, 1)), ());
}

#[test]
fn composite_field_access() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_price_tag) VALUES ('{}')",
        literal::price_tag()
    ));

    let expected: Query<(), (Nullable<Numeric>, Nullable<Text>, Nullable<PriceTag>)> =
        sql("SELECT (v_price_tag).amount, (v_price_tag).currency, v_price_tag FROM type_samples");
    case.assert_same(expected, compile(dsl::composite_field_access), ());
}

#[test]
fn composite_constructor_cast() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (PriceTag, Nullable<Text>)> = sql(
        "SELECT ROW(price_cents / 100.0, 'USD')::price_tag, (ROW(1.25, 'EUR')::price_tag).currency
         FROM products
         ORDER BY name",
    );
    case.assert_same(expected, compile(dsl::composite_constructor_cast), ());
}

#[test]
fn composite_expansion() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_price_tag) VALUES ('{}')",
        literal::price_tag()
    ));

    let expected: Query<(), (Nullable<Numeric>, Nullable<Text>)> =
        sql("SELECT (v_price_tag).* FROM type_samples");
    case.assert_same_named(expected, compile(dsl::composite_expansion), ());
}
