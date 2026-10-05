mod dsl;

use crate::support::{Case, rnd};
use postgres::types::Type;

#[test]
fn user_returning() {
    let mut case = Case::new();
    let name = rnd::text();
    let email = rnd::text();

    case.assert_same(
        &format!(
            "INSERT INTO users (name, email) VALUES ('{name}', '{email}') RETURNING name, email"
        ),
        || dsl::user_returning(&name, &email),
    );
}

#[test]
fn product_skips_generated_columns() {
    let mut case = Case::new();
    let name = rnd::text();
    let price_cents = rnd::int();
    let quantity = rnd::int();

    case.assert_same(
        &format!(
            "INSERT INTO products (name, price_cents, quantity)
             VALUES ('{name}', {price_cents}, {quantity})
             RETURNING name, price_cents, quantity, total_cents"
        ),
        || dsl::product(&name, price_cents, quantity),
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

    case.assert_same(
        &format!(
            "INSERT INTO groups (name)
             SELECT name FROM users WHERE name LIKE '{prefix}%' ORDER BY name
             RETURNING name"
        ),
        || dsl::insert_select(&format!("{prefix}%")),
    );
}

#[test]
fn multi_row_values() {
    let mut case = Case::new();
    let first = rnd::text();
    let second = rnd::text();
    let email = rnd::text();

    case.assert_same(
        &format!(
            "INSERT INTO users (name, email) VALUES ('{first}', '{email}'), ('{second}', NULL)
             RETURNING name, email"
        ),
        || dsl::multi_row_values(&first, &email, &second),
    );
}

#[test]
fn default_value() {
    let mut case = Case::new();
    let name = rnd::text();
    let price_cents = rnd::int();

    case.assert_same(
        &format!(
            "INSERT INTO products (name, price_cents, quantity) VALUES ('{name}', {price_cents}, DEFAULT)
             RETURNING name, price_cents, quantity"
        ),
        || dsl::default_value(&name, price_cents),
    );
}

#[test]
fn overriding_system_value() {
    let mut case = Case::new();
    let id = 1_000_000_000_000 + rnd::u64() as i64 % 1_000_000_000;
    let name = rnd::text();

    case.assert_same(
        &format!(
            "INSERT INTO products (id, name, price_cents) OVERRIDING SYSTEM VALUE
             VALUES ({id}, '{name}', 1)
             RETURNING id, name"
        ),
        || dsl::overriding_system_value(id, &name),
    );
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

    case.assert_same(
        &format!(
            "INSERT INTO user_settings (user_id, theme)
             SELECT id, 'light' FROM users WHERE name IN ('{taken}', '{free}')
             ON CONFLICT (user_id) DO NOTHING
             RETURNING user_id, theme"
        ),
        || dsl::on_conflict_do_nothing(&taken, &free),
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

    case.assert_same(
        &format!(
            "INSERT INTO user_settings (user_id, theme)
             SELECT id, 'dark' FROM users WHERE name = '{name}'
             ON CONFLICT (user_id) DO UPDATE SET theme = EXCLUDED.theme || '!'
                 WHERE user_settings.theme <> EXCLUDED.theme
             RETURNING user_id, theme"
        ),
        || dsl::on_conflict_do_update(&name, "dark"),
    );
}

#[test]
fn default_values() {
    let mut case = Case::new();

    case.assert_same(
        "INSERT INTO type_samples DEFAULT VALUES RETURNING v_int4 IS NULL, v_text IS NULL",
        dsl::default_values,
    );
}

#[test]
fn virtual_generated_column() {
    let mut case = Case::new();
    let name = rnd::text();
    let price_cents = rnd::int();

    case.assert_same(
        &format!(
            "INSERT INTO products (name, price_cents) VALUES ('{name}', {price_cents})
             RETURNING price_cents, price_with_tax, total_cents"
        ),
        || dsl::virtual_generated_column(&name, price_cents),
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

    case.assert_same(
        &format!(
            "INSERT INTO users (name, email) VALUES ('{renamed}', '{email}')
             ON CONFLICT (email) WHERE email IS NOT NULL DO UPDATE SET name = EXCLUDED.name
             RETURNING name, email"
        ),
        || dsl::on_conflict_partial_index(&renamed, &email),
    );
}

#[test]
fn returning_expression() {
    let mut case = Case::new();
    let name = rnd::text();
    let price_cents = rnd::int();
    let quantity = rnd::int();

    case.assert_same_named(
        &format!(
            "INSERT INTO products (name, price_cents, quantity)
             VALUES ('{name}', {price_cents}, {quantity})
             RETURNING price_cents * quantity AS total"
        ),
        || dsl::returning_expression(&name, price_cents, quantity),
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

    case.assert_same(
        "INSERT INTO groups (name) SELECT name || '-copy' FROM groups RETURNING name",
        dsl::insert_select_expression,
    );
}

#[test]
fn insert_bound() {
    let mut case = Case::new();
    let name = rnd::text();
    let email = rnd::text();

    case.assert_same(
        &format!("INSERT INTO users (name, email) VALUES ('{name}', NULL) RETURNING name, email"),
        || dsl::insert_bound(&name, None),
    );
    case.assert_param_types(&[Type::TEXT, Type::TEXT], || {
        dsl::insert_bound(&name, Some(&email))
    });
}
