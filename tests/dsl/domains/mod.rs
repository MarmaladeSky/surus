mod dsl;

use crate::schema::PositiveInt;
use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Nullable, Regtype};

#[test]
fn cast_to_domain() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{}', {}, {})",
        rnd::text(),
        rnd::int(),
        rnd::int(),
    ));

    let expected: Query<(), (PositiveInt, PositiveInt, Regtype)> = sql(
        "SELECT (quantity + 1)::positive_int, CAST(price_cents + 1 AS positive_int), pg_typeof((quantity + 1)::positive_int)
         FROM products",
    );
    case.assert_same(expected, compile(dsl::cast_to_domain), ());
}

#[test]
fn check_violation_on_cast() {
    let mut case = Case::new();

    let expected: Query<(), (PositiveInt,)> = sql("SELECT 0::positive_int");
    case.assert_same_error(expected, compile(dsl::check_violation_on_cast), ());
}

#[test]
fn check_violation_on_insert() {
    let mut case = Case::new();

    let expected: Query<(), (Nullable<PositiveInt>,)> =
        sql("INSERT INTO type_samples (v_positive_int) VALUES (0) RETURNING v_positive_int");
    case.assert_same_error(expected, compile(|| dsl::check_violation_on_insert(0)), ());
}
