mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Array, Bool, Float8, Int4, Int8, Nullable, Numeric, Text};

#[test]
fn count_per_group() {
    let mut case = Case::new();
    let big = rnd::text();
    let small = rnd::text();

    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{big}'), ('{small}');
         INSERT INTO users (name, group_id)
             SELECT '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', id FROM groups WHERE name = '{small}';",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(), (Nullable<Int8>, Int8)> =
        sql("SELECT group_id, count(*) FROM users GROUP BY group_id ORDER BY group_id");
    case.assert_same(expected, compile(dsl::count_per_group), ());
}

fn seed_mixed(case: &mut Case) {
    let big = rnd::text();
    let small = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{big}'), ('{small}');
         INSERT INTO users (name, email, group_id)
             SELECT '{}', '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', NULL, id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', NULL, id FROM groups WHERE name = '{small}';",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));
}

#[test]
fn having() {
    let mut case = Case::new();
    seed_mixed(&mut case);

    let expected: Query<(), (Nullable<Int8>, Int8)> = sql(
        "SELECT group_id, count(*) FROM users GROUP BY group_id HAVING count(*) > 1 ORDER BY group_id",
    );
    case.assert_same(expected, compile(dsl::having), ());
}

#[test]
fn rollup() {
    let mut case = Case::new();
    seed_mixed(&mut case);

    let expected: Query<(), (Nullable<Int8>, Nullable<Bool>, Int8)> =
        sql("SELECT group_id, email IS NULL, count(*)
         FROM users
         GROUP BY ROLLUP (group_id, (email IS NULL))
         ORDER BY 1, 2");
    case.assert_same(expected, compile(dsl::rollup), ());
}

#[test]
fn cube() {
    let mut case = Case::new();
    seed_mixed(&mut case);

    let expected: Query<(), (Nullable<Int8>, Nullable<Bool>, Int8)> =
        sql("SELECT group_id, email IS NULL, count(*)
         FROM users
         GROUP BY CUBE (group_id, (email IS NULL))
         ORDER BY 1, 2");
    case.assert_same(expected, compile(dsl::cube), ());
}

#[test]
fn grouping_sets() {
    let mut case = Case::new();
    seed_mixed(&mut case);

    let expected: Query<(), (Nullable<Int8>, Nullable<Bool>, Int8, Int4)> = sql(
        "SELECT group_id, email IS NULL, count(*), GROUPING(group_id, (email IS NULL))
         FROM users
         GROUP BY GROUPING SETS ((group_id), ((email IS NULL)), ())
         ORDER BY 1, 2",
    );
    case.assert_same(expected, compile(dsl::grouping_sets), ());
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
fn common_aggregates() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<
        (),
        (
            Nullable<Int8>,
            Nullable<Numeric>,
            Nullable<Text>,
            Nullable<Text>,
            Nullable<Bool>,
        ),
    > = sql(
        "SELECT sum(price_cents), avg(price_cents), min(name), max(name), bool_or(quantity > 1)
         FROM products",
    );
    case.assert_same(expected, compile(dsl::common_aggregates), ());
}

#[test]
fn filter_clause() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Int8, Nullable<Int8>)> = sql(
        "SELECT count(*) FILTER (WHERE quantity > 1), sum(price_cents) FILTER (WHERE quantity = 1)
         FROM products",
    );
    case.assert_same(expected, compile(dsl::filter_clause), ());
}

#[test]
fn distinct_inside() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Int8, Nullable<Array<Int4>>)> = sql(
        "SELECT count(DISTINCT quantity), array_agg(DISTINCT quantity ORDER BY quantity) FROM products",
    );
    case.assert_same(expected, compile(dsl::distinct_inside), ());
}

#[test]
fn order_by_inside() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Nullable<Text>, Nullable<Array<Int4>>)> = sql(
        "SELECT string_agg(name, ',' ORDER BY price_cents DESC), array_agg(price_cents ORDER BY price_cents)
         FROM products",
    );
    case.assert_same(expected, compile(dsl::order_by_inside), ());
}

#[test]
fn ordered_set() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Nullable<Float8>, Nullable<Int4>, Nullable<Int4>)> = sql(
        "SELECT percentile_cont(0.5) WITHIN GROUP (ORDER BY price_cents),
                percentile_disc(0.5) WITHIN GROUP (ORDER BY price_cents),
                mode() WITHIN GROUP (ORDER BY quantity)
         FROM products",
    );
    case.assert_same(expected, compile(dsl::ordered_set), ());
}

#[test]
fn hypothetical_set() {
    let mut case = Case::new();
    seed_products(&mut case);

    let expected: Query<(), (Int8, Int8, Float8, Float8)> =
        sql("SELECT rank(500) WITHIN GROUP (ORDER BY price_cents),
                dense_rank(500) WITHIN GROUP (ORDER BY price_cents),
                percent_rank(500) WITHIN GROUP (ORDER BY price_cents),
                cume_dist(500) WITHIN GROUP (ORDER BY price_cents)
         FROM products");
    case.assert_same(expected, compile(|| dsl::hypothetical_set(500)), ());
}
