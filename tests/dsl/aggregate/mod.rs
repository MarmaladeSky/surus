mod dsl;

use crate::support::{Case, rnd};

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

    case.assert_same(
        "SELECT group_id, count(*) FROM users GROUP BY group_id ORDER BY group_id",
        dsl::count_per_group,
    );
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

    case.assert_same(
        "SELECT group_id, count(*) FROM users GROUP BY group_id HAVING count(*) > 1 ORDER BY group_id",
        dsl::having,
    );
}

#[test]
fn rollup() {
    let mut case = Case::new();
    seed_mixed(&mut case);

    case.assert_same(
        "SELECT group_id, email IS NULL, count(*)
         FROM users
         GROUP BY ROLLUP (group_id, (email IS NULL))
         ORDER BY 1, 2",
        dsl::rollup,
    );
}

#[test]
fn cube() {
    let mut case = Case::new();
    seed_mixed(&mut case);

    case.assert_same(
        "SELECT group_id, email IS NULL, count(*)
         FROM users
         GROUP BY CUBE (group_id, (email IS NULL))
         ORDER BY 1, 2",
        dsl::cube,
    );
}

#[test]
fn grouping_sets() {
    let mut case = Case::new();
    seed_mixed(&mut case);

    case.assert_same(
        "SELECT group_id, email IS NULL, count(*), GROUPING(group_id, (email IS NULL))
         FROM users
         GROUP BY GROUPING SETS ((group_id), ((email IS NULL)), ())
         ORDER BY 1, 2",
        dsl::grouping_sets,
    );
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

    case.assert_same(
        "SELECT sum(price_cents), avg(price_cents), min(name), max(name), bool_or(quantity > 1)
         FROM products",
        dsl::common_aggregates,
    );
}

#[test]
fn filter_clause() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT count(*) FILTER (WHERE quantity > 1), sum(price_cents) FILTER (WHERE quantity = 1)
         FROM products",
        dsl::filter_clause,
    );
}

#[test]
fn distinct_inside() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT count(DISTINCT quantity), array_agg(DISTINCT quantity ORDER BY quantity) FROM products",
        dsl::distinct_inside,
    );
}

#[test]
fn order_by_inside() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT string_agg(name, ',' ORDER BY price_cents DESC), array_agg(price_cents ORDER BY price_cents)
         FROM products",
        dsl::order_by_inside,
    );
}

#[test]
fn ordered_set() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT percentile_cont(0.5) WITHIN GROUP (ORDER BY price_cents),
                percentile_disc(0.5) WITHIN GROUP (ORDER BY price_cents),
                mode() WITHIN GROUP (ORDER BY quantity)
         FROM products",
        dsl::ordered_set,
    );
}

#[test]
fn hypothetical_set() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT rank(500) WITHIN GROUP (ORDER BY price_cents),
                dense_rank(500) WITHIN GROUP (ORDER BY price_cents),
                percent_rank(500) WITHIN GROUP (ORDER BY price_cents),
                cume_dist(500) WITHIN GROUP (ORDER BY price_cents)
         FROM products",
        || dsl::hypothetical_set(500),
    );
}
