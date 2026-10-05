mod dsl;

use crate::support::{Case, rnd};
use postgres::types::Type;

#[test]
fn set_email() {
    let mut case = Case::new();
    let name = rnd::text();
    let email = rnd::text();

    case.exec(&format!("INSERT INTO users (name) VALUES ('{name}')"));

    case.assert_same(
        &format!("UPDATE users SET email = '{email}' WHERE name = '{name}' RETURNING name, email"),
        || dsl::set_email(&name, &email),
    );
}

#[test]
fn update_from() {
    let mut case = Case::new();
    let group = rnd::text();

    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{group}');
         INSERT INTO users (name, group_id)
             SELECT '{}', id FROM groups WHERE name = '{group}'
             UNION ALL SELECT '{}', NULL;",
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "UPDATE users SET email = groups.name || '@example.com'
         FROM groups
         WHERE groups.id = users.group_id
         RETURNING users.name, users.email",
        dsl::update_from,
    );
}

#[test]
fn set_row_from_subquery() {
    let mut case = Case::new();
    let user = rnd::text();
    let group = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{group}');
         INSERT INTO users (name) VALUES ('{user}'), ('{}');",
        rnd::text()
    ));

    case.assert_same(
        &format!(
            "UPDATE users
             SET (email, group_id) = (SELECT g.name || '@example.com', g.id FROM groups g WHERE g.name = '{group}')
             WHERE name = '{user}'
             RETURNING name, email, group_id IS NOT NULL"
        ),
        || dsl::set_row_from_subquery(&user, &group),
    );
}

#[test]
fn set_row_constructor() {
    let mut case = Case::new();
    let user = rnd::text();
    let renamed = rnd::text();
    let email = rnd::text();
    case.exec(&format!("INSERT INTO users (name) VALUES ('{user}')"));

    case.assert_same(
        &format!(
            "UPDATE users SET (name, email) = ROW('{renamed}', '{email}') WHERE name = '{user}'
             RETURNING name, email"
        ),
        || dsl::set_row_constructor(&user, &renamed, &email),
    );
}

#[test]
fn returning_star() {
    let mut case = Case::new();
    let name = rnd::text();
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{name}', {}, {})",
        rnd::int(),
        rnd::int(),
    ));

    case.assert_same_named(
        &format!("UPDATE products SET quantity = quantity + 1 WHERE name = '{name}' RETURNING *"),
        || dsl::returning_star(&name),
    );
}

#[test]
fn returning_old_and_new() {
    let mut case = Case::new();
    let name = rnd::text();
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{name}', {}, {})",
        rnd::int(),
        rnd::int(),
    ));

    case.assert_same(
        &format!(
            "UPDATE products SET quantity = quantity + 5 WHERE name = '{name}'
             RETURNING WITH (OLD AS o, NEW AS n) o.quantity, n.quantity, n.total_cents - o.total_cents"
        ),
        || dsl::returning_old_and_new(&name, 5),
    );
}

#[test]
fn update_set_expression() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{}', 5, 1), ('{}', 0, 3), ('{}', 9, 4)",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "UPDATE products SET quantity = quantity * 2 + 1 WHERE price_cents > 0 RETURNING name, quantity",
        dsl::update_set_expression,
    );
}

#[test]
fn update_from_join_predicate() {
    let mut case = Case::new();
    let group = rnd::text();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{group}');
         INSERT INTO users (name, email, group_id)
             SELECT '{}', NULL, id FROM groups WHERE name = '{group}'
             UNION ALL SELECT '{}', '{}', id FROM groups WHERE name = '{group}'
             UNION ALL SELECT '{}', NULL, NULL;",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    case.assert_same(
        "UPDATE users u SET email = g.name || '@x'
         FROM groups g
         WHERE g.id = u.group_id AND u.email IS NULL
         RETURNING u.name, u.email",
        dsl::update_from_join_predicate,
    );
}

#[test]
fn update_bound() {
    let mut case = Case::new();
    let id = 900_000_000 + (rnd::u64() % 100_000_000) as i64;
    let name = rnd::text();
    case.exec(&format!(
        "INSERT INTO users (id, name) VALUES ({id}, '{}'), ({}, '{}')",
        rnd::text(),
        id + 1,
        rnd::text(),
    ));

    case.assert_same(
        &format!("UPDATE users SET name = '{name}' WHERE id = {id} RETURNING id, name"),
        || dsl::update_bound(id, &name),
    );
    case.assert_param_types(&[Type::TEXT, Type::INT8], || dsl::update_bound(id, &name));
}
