mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Bool, Int4, Int8, Nullable, Text};

#[test]
fn set_email() {
    let mut case = Case::new();
    let name = rnd::text();
    let email = rnd::text();

    case.exec(&format!("INSERT INTO users (name) VALUES ('{name}')"));

    let expected: Query<(Text, Text), (Text, Nullable<Text>)> =
        sql("UPDATE users SET email = $1 WHERE name = $2 RETURNING name, email");
    case.assert_same(expected, compile(dsl::set_email), (&email, &name));
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

    let expected: Query<(), (Text, Nullable<Text>)> =
        sql("UPDATE users SET email = groups.name || '@example.com'
         FROM groups
         WHERE groups.id = users.group_id
         RETURNING users.name, users.email");
    case.assert_same(expected, compile(dsl::update_from), ());
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

    let expected: Query<(Text, Text), (Text, Nullable<Text>, Bool)> = sql(
        "UPDATE users
         SET (email, group_id) = (SELECT g.name || '@example.com', g.id FROM groups g WHERE g.name = $1)
         WHERE name = $2
         RETURNING name, email, group_id IS NOT NULL",
    );
    case.assert_same(
        expected,
        compile(dsl::set_row_from_subquery),
        (&group, &user),
    );
}

#[test]
fn set_row_constructor() {
    let mut case = Case::new();
    let user = rnd::text();
    let renamed = rnd::text();
    let email = rnd::text();
    case.exec(&format!("INSERT INTO users (name) VALUES ('{user}')"));

    let expected: Query<(Text, Text, Text), (Text, Nullable<Text>)> = sql(
        "UPDATE users SET (name, email) = ROW($1, $2) WHERE name = $3
         RETURNING name, email",
    );
    case.assert_same(
        expected,
        compile(dsl::set_row_constructor),
        (&renamed, &email, &user),
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

    let expected: Query<(Text,), (Int8, Text, Int4, Int4, Nullable<Int8>, Nullable<Int8>)> =
        sql("UPDATE products SET quantity = quantity + 1 WHERE name = $1 RETURNING *");
    case.assert_same_named(expected, compile(dsl::returning_star), (&name,));
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

    let expected: Query<(Text,), (Int4, Int4, Nullable<Int8>)> = sql(
        "UPDATE products SET quantity = quantity + 5 WHERE name = $1
         RETURNING WITH (OLD AS o, NEW AS n) o.quantity, n.quantity, n.total_cents - o.total_cents",
    );
    case.assert_same(
        expected,
        compile(|| dsl::returning_old_and_new(5)),
        (&name,),
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

    let expected: Query<(), (Text, Int4)> = sql(
        "UPDATE products SET quantity = quantity * 2 + 1 WHERE price_cents > 0 RETURNING name, quantity",
    );
    case.assert_same(expected, compile(dsl::update_set_expression), ());
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

    let expected: Query<(), (Text, Nullable<Text>)> =
        sql("UPDATE users u SET email = g.name || '@x'
         FROM groups g
         WHERE g.id = u.group_id AND u.email IS NULL
         RETURNING u.name, u.email");
    case.assert_same(expected, compile(dsl::update_from_join_predicate), ());
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

    let expected: Query<(Text, Int8), (Int8, Text)> =
        sql("UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name");
    case.assert_same(expected, compile(dsl::update_bound), (&name, id));
}
