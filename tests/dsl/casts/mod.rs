mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Bool, Float8, Int4, Int8, Numeric, Text, Varchar};

fn seed_products(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{}', {}, {}), ('{}', {}, 0)",
        rnd::text(),
        rnd::int(),
        1 + rnd::int(),
        rnd::text(),
        rnd::int(),
    ));
}

#[test]
fn double_colon() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text, Numeric, Bool)> = sql(
        "SELECT price_cents::text, price_cents::numeric / 100, quantity::bool FROM products ORDER BY name",
    );
    case.assert_same(expected, compile(dsl::double_colon), ());
}

#[test]
fn cast_function() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Int8, Varchar, Float8)> = sql(
        "SELECT CAST(price_cents AS int8), CAST(name AS varchar(5)), CAST(quantity AS float8) FROM products ORDER BY name",
    );
    case.assert_same(expected, compile(dsl::cast_function), ());
}

#[test]
fn invalid_text_fails_at_runtime() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Int4,)> = sql("SELECT name::int4 FROM products");
    case.assert_same_error(expected, compile(dsl::invalid_text_fails_at_runtime), ());
}
