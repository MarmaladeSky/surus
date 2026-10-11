mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Bool, Int4, Int8, Nullable, Text};

#[test]
fn user_returning() {
    let mut case = Case::new();
    let name = rnd::text();
    let email = rnd::text();

    let expected: Query<(Text, Text), (Text, Nullable<Text>)> =
        sql("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING name, email");
    case.assert_same(expected, compile(dsl::user_returning), (&name, &email));
}

#[test]
fn product_skips_generated_columns() {
    let mut case = Case::new();
    let name = rnd::text();
    let price_cents = rnd::int();
    let quantity = rnd::int();

    let expected: Query<(Text, Int4, Int4), (Text, Int4, Int4, Nullable<Int8>)> =
        sql("INSERT INTO products (name, price_cents, quantity)
         VALUES ($1, $2, $3)
         RETURNING name, price_cents, quantity, total_cents");
    case.assert_same(
        expected,
        compile(dsl::product),
        (&name, price_cents, quantity),
    );
}

#[test]
fn insert_select() {
    let mut case = Case::new();
    let prefix = rnd::text();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{prefix}_a'), ('{prefix}_b'), ('{}')",
        rnd::text()
    ));

    let expected: Query<(Text,), (Text,)> = sql("INSERT INTO groups (name)
         SELECT name FROM users WHERE name LIKE $1 ORDER BY name
         RETURNING name");
    case.assert_same(
        expected,
        compile(dsl::insert_select),
        (format!("{prefix}%"),),
    );
}

#[test]
fn multi_row_values() {
    let mut case = Case::new();
    let first = rnd::text();
    let second = rnd::text();
    let email = rnd::text();

    let expected: Query<(Text, Text, Text), (Text, Nullable<Text>)> =
        sql("INSERT INTO users (name, email) VALUES ($1, $2), ($3, NULL)
         RETURNING name, email");
    case.assert_same(
        expected,
        compile(dsl::multi_row_values),
        (&first, &email, &second),
    );
}

#[test]
fn default_value() {
    let mut case = Case::new();
    let name = rnd::text();
    let price_cents = rnd::int();

    let expected: Query<(Text, Int4), (Text, Int4, Int4)> = sql(
        "INSERT INTO products (name, price_cents, quantity) VALUES ($1, $2, DEFAULT)
         RETURNING name, price_cents, quantity",
    );
    case.assert_same(expected, compile(dsl::default_value), (&name, price_cents));
}

#[test]
fn overriding_system_value() {
    let mut case = Case::new();
    let id = 1_000_000_000_000 + rnd::u64() as i64 % 1_000_000_000;
    let name = rnd::text();

    let expected: Query<(Int8, Text), (Int8, Text)> = sql(
        "INSERT INTO products (id, name, price_cents) OVERRIDING SYSTEM VALUE
         VALUES ($1, $2, 1)
         RETURNING id, name",
    );
    case.assert_same(expected, compile(dsl::overriding_system_value), (id, &name));
}

#[test]
fn on_conflict_do_nothing() {
    let mut case = Case::new();
    let taken = rnd::text();
    let free = rnd::text();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{taken}'), ('{free}');
         INSERT INTO user_settings (user_id, theme)
             SELECT id, '{}' FROM users WHERE name = '{taken}';",
        rnd::text()
    ));

    let expected: Query<(Text, Text), (Int8, Text)> =
        sql("INSERT INTO user_settings (user_id, theme)
         SELECT id, 'light' FROM users WHERE name IN ($1, $2)
         ON CONFLICT (user_id) DO NOTHING
         RETURNING user_id, theme");
    case.assert_same(
        expected,
        compile(dsl::on_conflict_do_nothing),
        (&taken, &free),
    );
}

#[test]
fn on_conflict_do_update() {
    let mut case = Case::new();
    let name = rnd::text();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{name}');
         INSERT INTO user_settings (user_id, theme) SELECT id, 'light' FROM users WHERE name = '{name}';"
    ));

    let expected: Query<(Text,), (Int8, Text)> = sql("INSERT INTO user_settings (user_id, theme)
         SELECT id, 'dark' FROM users WHERE name = $1
         ON CONFLICT (user_id) DO UPDATE SET theme = EXCLUDED.theme || '!'
             WHERE user_settings.theme <> EXCLUDED.theme
         RETURNING user_id, theme");
    case.assert_same(
        expected,
        compile(|| dsl::on_conflict_do_update("dark")),
        (&name,),
    );
}

#[test]
fn default_values() {
    let mut case = Case::new();

    let expected: Query<(), (Bool, Bool)> =
        sql("INSERT INTO type_samples DEFAULT VALUES RETURNING v_int4 IS NULL, v_text IS NULL");
    case.assert_same(expected, compile(dsl::default_values), ());
}

#[test]
fn virtual_generated_column() {
    let mut case = Case::new();
    let name = rnd::text();
    let price_cents = rnd::int();

    let expected: Query<(Text, Int4), (Int4, Nullable<Int8>, Nullable<Int8>)> =
        sql("INSERT INTO products (name, price_cents) VALUES ($1, $2)
         RETURNING price_cents, price_with_tax, total_cents");
    case.assert_same(
        expected,
        compile(dsl::virtual_generated_column),
        (&name, price_cents),
    );
}

#[test]
fn on_conflict_partial_index() {
    let mut case = Case::new();
    let email = rnd::text();
    let renamed = rnd::text();
    case.exec(&format!(
        "INSERT INTO users (name, email) VALUES ('{}', '{email}'), ('{}', NULL), ('{}', NULL)",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(Text, Text), (Text, Nullable<Text>)> =
        sql("INSERT INTO users (name, email) VALUES ($1, $2)
         ON CONFLICT (email) WHERE email IS NOT NULL DO UPDATE SET name = EXCLUDED.name
         RETURNING name, email");
    case.assert_same(
        expected,
        compile(dsl::on_conflict_partial_index),
        (&renamed, &email),
    );
}

#[test]
fn returning_expression() {
    let mut case = Case::new();
    let name = rnd::text();
    let price_cents = rnd::int();
    let quantity = rnd::int();

    let expected: Query<(Text, Int4, Int4), (Int4,)> =
        sql("INSERT INTO products (name, price_cents, quantity)
         VALUES ($1, $2, $3)
         RETURNING price_cents * quantity AS total");
    case.assert_same_named(
        expected,
        compile(dsl::returning_expression),
        (&name, price_cents, quantity),
    );
}

#[test]
fn insert_select_expression() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO groups (name) VALUES ('{}'), ('{}')",
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(), (Text,)> =
        sql("INSERT INTO groups (name) SELECT name || '-copy' FROM groups RETURNING name");
    case.assert_same(expected, compile(dsl::insert_select_expression), ());
}

#[test]
fn insert_bound() {
    let mut case = Case::new();
    let name = rnd::text();

    let expected: Query<(Text, Nullable<Text>), (Text, Nullable<Text>)> =
        sql("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING name, email");
    case.assert_same(expected, compile(dsl::insert_bound), (&name, None::<&str>));
}
