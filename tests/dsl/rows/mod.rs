mod dsl;

use crate::support::{Case, literal, rnd};

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

    case.assert_same(
        "SELECT ROW(name, price_cents), (quantity, name) FROM products ORDER BY name",
        dsl::constructor,
    );
}

#[test]
fn comparison() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT name FROM products WHERE (price_cents, quantity) > (500, 1) ORDER BY name",
        || dsl::comparison(500, 1),
    );
}

#[test]
fn composite_field_access() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_price_tag) VALUES ('{}')",
        literal::price_tag()
    ));

    case.assert_same(
        "SELECT (v_price_tag).amount, (v_price_tag).currency, v_price_tag FROM type_samples",
        dsl::composite_field_access,
    );
}

#[test]
fn composite_constructor_cast() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT ROW(price_cents / 100.0, 'USD')::price_tag, (ROW(1.25, 'EUR')::price_tag).currency
         FROM products
         ORDER BY name",
        dsl::composite_constructor_cast,
    );
}

#[test]
fn composite_expansion() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_price_tag) VALUES ('{}')",
        literal::price_tag()
    ));

    case.assert_same_named(
        "SELECT (v_price_tag).* FROM type_samples",
        dsl::composite_expansion,
    );
}
