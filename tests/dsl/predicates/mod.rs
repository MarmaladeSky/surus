mod dsl;

use crate::support::{Case, rnd};

struct Users {
    alpha: String,
    gamma: String,
}

fn seed_users(case: &mut Case) -> Users {
    let users = Users {
        alpha: format!("alpha_{}", rnd::text()),
        gamma: format!("gamma_{}", rnd::text()),
    };
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{}');
         INSERT INTO users (name, email, group_id)
             SELECT '{}', '{}', id FROM groups
             UNION ALL SELECT 'Beta_{}', NULL, id FROM groups
             UNION ALL SELECT '{}', '{}', NULL
             UNION ALL SELECT 'delta_{}', NULL, NULL;",
        rnd::text(),
        users.alpha,
        rnd::text(),
        rnd::text(),
        users.gamma,
        rnd::text(),
        rnd::text(),
    ));
    users
}

fn seed_products(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity)
         VALUES ('{}', {}, 0), ('{}', {}, 1), ('{}', {}, 5)",
        rnd::text(),
        rnd::u64() % 300,
        rnd::text(),
        300 + rnd::u64() % 300,
        rnd::text(),
        600 + rnd::u64() % 300,
    ));
}

#[test]
fn and_or_not() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT name FROM users
         WHERE (email IS NOT NULL AND group_id IS NOT NULL)
            OR NOT (name LIKE 'alpha%' OR name LIKE 'Beta%')
         ORDER BY name",
        dsl::and_or_not,
    );
}

#[test]
fn is_null() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT name, email IS NULL, group_id IS NOT NULL FROM users ORDER BY name",
        dsl::is_null,
    );
}

#[test]
fn is_distinct_from() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT a.name, b.name, a.group_id IS NOT DISTINCT FROM b.group_id
         FROM users a JOIN users b ON a.name < b.name
         WHERE a.email IS DISTINCT FROM b.email
         ORDER BY 1, 2",
        dsl::is_distinct_from,
    );
}

#[test]
fn is_true() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT name, (email IS NULL) IS TRUE, (group_id > 0) IS NOT FALSE, (group_id > 0) IS UNKNOWN
         FROM users
         ORDER BY name",
        dsl::is_true,
    );
}

#[test]
fn between() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT name FROM products WHERE price_cents BETWEEN 300 AND 599 ORDER BY name",
        || dsl::between(300, 599),
    );
}

#[test]
fn in_list() {
    let mut case = Case::new();
    let users = seed_users(&mut case);

    case.assert_same(
        &format!(
            "SELECT name FROM users WHERE name IN ('{}', '{}') ORDER BY name",
            users.alpha, users.gamma
        ),
        || dsl::in_list(&users.alpha, &users.gamma),
    );
}

#[test]
fn like() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT name FROM users WHERE name LIKE 'alpha%' ORDER BY name",
        || dsl::like("alpha%"),
    );
}

#[test]
fn ilike() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT name FROM users WHERE name ILIKE 'beta%' ORDER BY name",
        || dsl::ilike("beta%"),
    );
}

#[test]
fn similar_to() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT name FROM users WHERE name SIMILAR TO '(alpha|gamma)%' ORDER BY name",
        || dsl::similar_to("(alpha|gamma)%"),
    );
}

#[test]
fn like_escape() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('100%_done_{}'), ('100x_done_{}'), ('{}')",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "SELECT name FROM users WHERE name LIKE '100!%!_done%' ESCAPE '!' ORDER BY name",
        || dsl::like_escape("100!%!_done%", "!"),
    );
}

#[test]
fn between_symmetric() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT name FROM products WHERE price_cents BETWEEN SYMMETRIC 599 AND 300 ORDER BY name",
        || dsl::between_symmetric(599, 300),
    );
}

#[test]
fn overlaps() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_date, v_timestamp) VALUES ('{}', '{}')",
        crate::support::literal::date(),
        crate::support::literal::timestamp(),
    ));

    case.assert_same(
        "SELECT (v_date, v_date + 10) OVERLAPS (v_date + 5, v_date + 20),
                (v_date, v_date + 10) OVERLAPS (v_date + 10, v_date + 20),
                (v_timestamp, INTERVAL '1 day') OVERLAPS (v_timestamp + INTERVAL '2 days', INTERVAL '1 day')
         FROM type_samples",
        dsl::overlaps,
    );
}
