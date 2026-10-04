mod dsl;

use crate::support::{Case, literal, rnd};

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

    case.assert_same(
        "SELECT ARRAY[price_cents, quantity * 100], ARRAY[name, 'fixed'] FROM products",
        dsl::constructor,
    );
}

#[test]
fn subscript_and_slice() {
    let mut case = Case::new();
    seed_sample(&mut case);

    case.assert_same(
        "SELECT v_int4_array[1], v_int4_array[2:3], cardinality(v_int4_array) FROM type_samples",
        dsl::subscript_and_slice,
    );
}

#[test]
fn unnest() {
    let mut case = Case::new();
    seed_sample(&mut case);

    case.assert_same(
        "SELECT unnest(v_text_array) FROM type_samples ORDER BY 1",
        dsl::unnest,
    );
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

    case.assert_same(
        &format!(
            "SELECT name FROM products WHERE price_cents = ANY (ARRAY[{wanted}, {other}]) ORDER BY name"
        ),
        || dsl::any_array_parameter(&[wanted, other]),
    );
}
