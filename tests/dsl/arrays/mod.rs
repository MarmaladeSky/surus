mod dsl;

use crate::support::{Case, compile, literal, rnd, sql};
use surus::Query;
use surus::types::{Array, Int4, Nullable, Text};

fn seed_sample(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO type_samples (v_int4_array, v_text_array) VALUES ('{}', '{}')",
        literal::int4_array(),
        literal::text_array(),
    ));
}

#[test]
fn constructor() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity) VALUES ('{}', {}, {})",
        rnd::text(),
        rnd::int(),
        rnd::int(),
    ));

    let expected: Query<(), (Array<Int4>, Array<Text>)> =
        sql("SELECT ARRAY[price_cents, quantity * 100], ARRAY[name, 'fixed'] FROM products");
    case.assert_same(expected, compile(dsl::constructor), ());
}

#[test]
fn subscript_and_slice() {
    let mut case = Case::new();
    seed_sample(&mut case);

    let expected: Query<(), (Nullable<Int4>, Nullable<Array<Int4>>, Nullable<Int4>)> = sql(
        "SELECT v_int4_array[1], v_int4_array[2:3], cardinality(v_int4_array) FROM type_samples",
    );
    case.assert_same(expected, compile(dsl::subscript_and_slice), ());
}

#[test]
fn unnest() {
    let mut case = Case::new();
    seed_sample(&mut case);

    let expected: Query<(), (Nullable<Text>,)> =
        sql("SELECT unnest(v_text_array) FROM type_samples ORDER BY 1");
    case.assert_same(expected, compile(dsl::unnest), ());
}

#[test]
fn any_array_parameter() {
    let mut case = Case::new();
    let wanted = rnd::int();
    let other = rnd::int();
    case.exec(&format!(
        "INSERT INTO products (name, price_cents, quantity)
         VALUES ('{}', {wanted}, 1), ('{}', {other}, 1), ('{}', {}, 1)",
        rnd::text(),
        rnd::text(),
        rnd::text(),
        wanted + other + 1,
    ));

    let expected: Query<(Array<Int4>,), (Text,)> =
        sql("SELECT name FROM products WHERE price_cents = ANY ($1) ORDER BY name");
    case.assert_same(
        expected,
        compile(dsl::any_array_parameter),
        (vec![wanted, other],),
    );
}
