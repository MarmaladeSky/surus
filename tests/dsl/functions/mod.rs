mod dsl;

use crate::support::{Case, literal, rnd};

fn seed_users(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO users (name, email) VALUES ('{}', '{}'), ('{}', NULL)",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));
}

fn seed_products(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{}', {}, {}), ('{}', {}, {})",
        rnd::text(),
        rnd::int(),
        rnd::int(),
        rnd::text(),
        rnd::int(),
        rnd::int(),
    ));
}

#[test]
fn string_functions() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT upper(name), length(name), substr(name, 2, 3), concat_ws('-', name, email), format('%s <%s>', name, email)
         FROM users
         ORDER BY name",
        dsl::string_functions,
    );
}

#[test]
fn math_functions() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT abs(price_cents - 500), round(price_cents / 7.0, 2), power(quantity, 2), mod(price_cents, 7)
         FROM products
         ORDER BY name",
        dsl::math_functions,
    );
}

#[test]
fn datetime_functions() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_date, v_timestamp) VALUES ('{}', '{}')",
        literal::date(),
        literal::timestamp(),
    ));

    case.assert_same(
        "SELECT date_trunc('month', v_timestamp), extract(year FROM v_date), to_char(v_date, 'YYYY-MM-DD')
         FROM type_samples",
        dsl::datetime_functions,
    );
}

#[test]
fn named_argument_notation() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT make_interval(days => quantity), make_date(year => 2024, month => 1, day => 1 + quantity % 28)
         FROM products
         ORDER BY name",
        dsl::named_argument_notation,
    );
}

#[test]
fn function_of_function() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT upper(substr(lower(name), 1, 3)) || '!' FROM users ORDER BY name",
        dsl::function_of_function,
    );
}

#[test]
fn aggregate_of_expression() {
    let mut case = Case::new();
    seed_users(&mut case);

    case.assert_same(
        "SELECT count(*) FILTER (WHERE email IS NOT NULL), sum(length(name) * 2) FROM users",
        dsl::aggregate_of_expression,
    );
}

#[test]
fn window_over_expression() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT name, sum(price_cents * quantity) OVER (ORDER BY name) FROM products ORDER BY name",
        dsl::window_over_expression,
    );
}
