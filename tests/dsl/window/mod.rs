mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Float8, Int4, Int8, Nullable, Numeric, Text};

#[test]
fn row_number_by_name() {
    let mut case = Case::new();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{}'), ('{}'), ('{}')",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(), (Text, Int8)> =
        sql("SELECT name, row_number() OVER (ORDER BY name) FROM users ORDER BY name");
    case.assert_same(expected, compile(dsl::row_number_by_name), ());
}

fn seed_products(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity)
         VALUES ('{}', {}, 1), ('{}', {}, 1), ('{}', {}, 2), ('{}', {}, 2), ('{}', {}, 2)",
        rnd::text(),
        rnd::int(),
        rnd::text(),
        rnd::int(),
        rnd::text(),
        rnd::int(),
        rnd::text(),
        rnd::int(),
        rnd::text(),
        rnd::int(),
    ));
}

#[test]
fn partition_by() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text, Int4, Int8, Nullable<Int8>, Int8)> =
        sql("SELECT name, quantity,
                rank() OVER (PARTITION BY quantity ORDER BY price_cents, name),
                sum(price_cents) OVER (PARTITION BY quantity),
                count(*) FILTER (WHERE price_cents > 500) OVER (PARTITION BY quantity)
         FROM products
         ORDER BY name");
    case.assert_same(expected, compile(dsl::partition_by), ());
}

#[test]
fn lag_lead() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text, Nullable<Int4>, Nullable<Int4>)> = sql("SELECT name,
                lag(price_cents) OVER (ORDER BY price_cents, name),
                lead(price_cents, 1, 0) OVER (ORDER BY price_cents, name)
         FROM products
         ORDER BY price_cents, name");
    case.assert_same(expected, compile(dsl::lag_lead), ());
}

#[test]
fn frame_clauses() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text, Nullable<Int8>, Nullable<Numeric>, Int8)> = sql(
        "SELECT name,
                sum(price_cents) OVER (ORDER BY price_cents, name ROWS BETWEEN 1 PRECEDING AND CURRENT ROW),
                avg(price_cents) OVER (ORDER BY price_cents, name RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW),
                count(*) OVER (ORDER BY quantity GROUPS BETWEEN CURRENT ROW AND 1 FOLLOWING)
         FROM products
         ORDER BY price_cents, name",
    );
    case.assert_same(expected, compile(dsl::frame_clauses), ());
}

#[test]
fn named_window() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text, Int8, Int8, Int4)> = sql(
        "SELECT name, rank() OVER w, dense_rank() OVER w, ntile(2) OVER w
         FROM products
         WINDOW w AS (PARTITION BY quantity ORDER BY price_cents, name)
         ORDER BY name",
    );
    case.assert_same(expected, compile(dsl::named_window), ());
}

#[test]
fn value_functions() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<
        (),
        (
            Text,
            Nullable<Text>,
            Nullable<Text>,
            Nullable<Text>,
            Float8,
            Float8,
        ),
    > = sql("SELECT name,
                first_value(name) OVER w,
                last_value(name) OVER (w ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING),
                nth_value(name, 2) OVER w,
                percent_rank() OVER w,
                cume_dist() OVER w
         FROM products
         WINDOW w AS (ORDER BY price_cents, name)
         ORDER BY price_cents, name");
    case.assert_same(expected, compile(dsl::value_functions), ());
}

#[test]
fn frame_exclusion_and_offsets() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Text, Nullable<Int8>, Nullable<Int8>, Int8, Int8)> = sql(
        "SELECT name,
                sum(price_cents) OVER (ORDER BY price_cents, name ROWS BETWEEN 1 PRECEDING AND 1 FOLLOWING EXCLUDE CURRENT ROW),
                sum(price_cents) OVER (ORDER BY price_cents RANGE BETWEEN 100 PRECEDING AND 100 FOLLOWING),
                count(*) OVER (ORDER BY quantity RANGE BETWEEN CURRENT ROW AND UNBOUNDED FOLLOWING EXCLUDE TIES),
                count(*) OVER (ORDER BY quantity ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING EXCLUDE GROUP)
         FROM products
         ORDER BY price_cents, name",
    );
    case.assert_same(expected, compile(dsl::frame_exclusion_and_offsets), ());
}
