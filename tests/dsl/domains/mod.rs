mod dsl;

use crate::support::{Case, rnd};

#[test]
fn cast_to_domain() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{}', {}, {})",
        rnd::text(),
        rnd::int(),
        rnd::int(),
    ));

    case.assert_same(
        "SELECT (quantity + 1)::positive_int, CAST(price_cents + 1 AS positive_int), pg_typeof((quantity + 1)::positive_int)
         FROM products",
        dsl::cast_to_domain,
    );
}

#[test]
fn check_violation_on_cast() {
    let mut case = Case::new();

    case.assert_same_error("SELECT 0::positive_int", dsl::check_violation_on_cast);
}

#[test]
fn check_violation_on_insert() {
    let mut case = Case::new();

    case.assert_same_error(
        "INSERT INTO type_samples (v_positive_int) VALUES (0) RETURNING v_positive_int",
        || dsl::check_violation_on_insert(0),
    );
}
