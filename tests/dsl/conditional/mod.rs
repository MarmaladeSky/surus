mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Int4, Nullable, Text};

fn seed_users(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{}');
         INSERT INTO users (name, email, group_id)
             SELECT '{}', '{}', id FROM groups
             UNION ALL SELECT '{}', NULL, id FROM groups
             UNION ALL SELECT '{}', '{}', NULL;",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));
}

fn seed_products(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity)
         VALUES ('{}', {}, 0), ('{}', {}, 1), ('{}', {}, 5)",
        rnd::text(),
        rnd::int(),
        rnd::text(),
        rnd::int(),
        rnd::text(),
        rnd::int(),
    ));
}

#[test]
fn case_searched() {
    let mut case = Case::new();
    seed_users(&mut case);

    let expected: Query<(), (Text, Text)> = sql("SELECT name,
                CASE WHEN email IS NULL THEN 'no email'
                     WHEN group_id IS NULL THEN 'no group'
                     ELSE 'complete' END
         FROM users
         ORDER BY name");
    case.assert_same(expected, compile(dsl::case_searched), ());
}

#[test]
fn case_simple() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text, Text)> = sql(
        "SELECT name, CASE quantity WHEN 0 THEN 'out' WHEN 1 THEN 'last' ELSE 'in stock' END
         FROM products
         ORDER BY name",
    );
    case.assert_same(expected, compile(dsl::case_simple), ());
}

#[test]
fn coalesce() {
    let mut case = Case::new();
    seed_users(&mut case);

    let expected: Query<(), (Text, Text)> =
        sql("SELECT name, coalesce(email, name) FROM users ORDER BY name");
    case.assert_same(expected, compile(dsl::coalesce), ());
}

#[test]
fn nullif() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text, Nullable<Int4>)> =
        sql("SELECT name, nullif(quantity, 0) FROM products ORDER BY name");
    case.assert_same(expected, compile(dsl::nullif), ());
}

#[test]
fn greatest_least() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text, Int4, Int4)> = sql(
        "SELECT name, greatest(price_cents, quantity * 100), least(price_cents, quantity * 100)
         FROM products
         ORDER BY name",
    );
    case.assert_same(expected, compile(dsl::greatest_least), ());
}

#[test]
fn case_into_arithmetic() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text, Int4)> = sql(
        "SELECT name, (CASE WHEN quantity > 0 THEN price_cents ELSE 0 END) + 1 FROM products ORDER BY name",
    );
    case.assert_same(expected, compile(dsl::case_into_arithmetic), ());
}
