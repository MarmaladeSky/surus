mod dsl;

use crate::support::{Case, rnd};

#[test]
fn scalar_group_name() {
    let mut case = Case::new();
    let group = rnd::text();
    let member = rnd::text();
    let loner = rnd::text();

    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{group}');
         INSERT INTO users (name, group_id)
             VALUES ('{member}', (SELECT id FROM groups WHERE name = '{group}'));
         INSERT INTO users (name) VALUES ('{loner}');"
    ));

    case.assert_same(
        "SELECT users.name, (SELECT groups.name FROM groups WHERE groups.id = users.group_id)
         FROM users
         ORDER BY users.name",
        dsl::scalar_group_name,
    );
}

#[test]
fn scalar_multiple_rows_fails() {
    let mut case = Case::new();

    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{}'), ('{}');
         INSERT INTO users (name) VALUES ('{}');",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same_error(
        "SELECT users.name, (SELECT groups.name FROM groups) FROM users",
        dsl::scalar_multiple_rows_fails,
    );
}

fn seed_groups(case: &mut Case) -> String {
    let big = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{big}'), ('{}');
         INSERT INTO users (name, group_id)
             SELECT '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', id FROM groups WHERE name = '{big}'
             UNION ALL SELECT '{}', NULL;",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));
    big
}

fn seed_products(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity)
         VALUES ('{}', {}, 1), ('{}', {}, 1), ('{}', {}, 1)",
        rnd::text(),
        rnd::int(),
        rnd::text(),
        1_000 + rnd::int(),
        rnd::text(),
        2_000 + rnd::int(),
    ));
}

#[test]
fn exists_groups_with_users() {
    let mut case = Case::new();
    seed_groups(&mut case);

    case.assert_same(
        "SELECT name FROM groups
         WHERE EXISTS (SELECT 1 FROM users WHERE users.group_id = groups.id)
         ORDER BY name",
        dsl::exists_groups_with_users,
    );
}

#[test]
fn not_exists_empty_groups() {
    let mut case = Case::new();
    seed_groups(&mut case);

    case.assert_same(
        "SELECT name FROM groups
         WHERE NOT EXISTS (SELECT 1 FROM users WHERE users.group_id = groups.id)
         ORDER BY name",
        dsl::not_exists_empty_groups,
    );
}

#[test]
fn in_subquery() {
    let mut case = Case::new();
    let big = seed_groups(&mut case);

    case.assert_same(
        &format!(
            "SELECT name FROM users
             WHERE group_id IN (SELECT id FROM groups WHERE name = '{big}')
             ORDER BY name"
        ),
        || dsl::in_subquery(&big),
    );
}

#[test]
fn any_not_the_cheapest() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT name FROM products
         WHERE price_cents > ANY (SELECT price_cents FROM products)
         ORDER BY name",
        dsl::any_not_the_cheapest,
    );
}

#[test]
fn all_the_most_expensive() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT name FROM products
         WHERE price_cents >= ALL (SELECT price_cents FROM products)
         ORDER BY name",
        dsl::all_the_most_expensive,
    );
}

#[test]
fn not_in_subquery() {
    let mut case = Case::new();
    let big = seed_groups(&mut case);
    case.exec(&format!(
        "INSERT INTO users (name, group_id) SELECT '{}', id FROM groups WHERE name <> '{big}'",
        rnd::text()
    ));

    case.assert_same(
        &format!(
            "SELECT name FROM users
             WHERE group_id NOT IN (SELECT id FROM groups WHERE name = '{big}')
             ORDER BY name"
        ),
        || dsl::not_in_subquery(&big),
    );
}

#[test]
fn not_in_with_null_returns_nothing() {
    let mut case = Case::new();
    seed_groups(&mut case);

    case.assert_same(
        "SELECT 'with null', count(*) FROM users
         WHERE group_id NOT IN (SELECT group_id FROM users)
         UNION ALL
         SELECT 'without null', count(*) FROM users
         WHERE group_id NOT IN (SELECT group_id FROM users WHERE group_id IS NOT NULL)",
        dsl::not_in_with_null_returns_nothing,
    );
}

#[test]
fn row_valued_in() {
    let mut case = Case::new();
    let big = seed_groups(&mut case);

    case.assert_same(
        &format!(
            "SELECT name FROM users
             WHERE (group_id, true) IN (SELECT id, name = '{big}' FROM groups)
             ORDER BY name"
        ),
        || dsl::row_valued_in(&big),
    );
}
