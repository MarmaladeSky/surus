mod dsl;

use crate::support::{Case, Input, compile, rnd, sql};
use surus::Query;
use surus::types::{Array, Bool, Float4, Float8, Int4, Int8, Jsonb, Jsonpath, Nullable, Text};

#[test]
fn typed_parameter_list() {
    let mut case = Case::new();

    let expected: Query<(Text, Int4, Int8, Bool), (Text, Int4, Int8, Bool)> =
        sql("SELECT $1::text, $2::int4, $3::int8, $4::bool");
    case.assert_same(
        expected,
        compile(dsl::typed_parameter_list),
        ("x", 1i32, 2i64, true),
    );
}

#[test]
fn many_parameters_in() {
    let mut case = Case::new();
    let names = seed_names(&mut case, 11);

    let expected: Query<(Text, Text, Text, Text, Text, Text, Text, Text, Text, Text), (Text,)> =
        sql(
            "SELECT name FROM users WHERE name IN ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) ORDER BY name",
        );
    let values = (
        &names[0], &names[1], &names[2], &names[3], &names[4], &names[5], &names[6], &names[7],
        &names[8], &names[9],
    );
    case.assert_same(expected, compile(dsl::many_parameters_in), values);
}

#[test]
fn many_parameters_any() {
    let mut case = Case::new();
    let names = seed_names(&mut case, 11);

    let expected: Query<(Array<Text>,), (Text,)> =
        sql("SELECT name FROM users WHERE name = ANY($1) ORDER BY name");
    case.assert_same(expected, compile(dsl::many_parameters_any), (&names[..10],));
}

#[test]
fn parameter_reuse() {
    let mut case = Case::new();
    let shared = rnd::text();
    case.exec(&format!(
        "INSERT INTO users (name, email) VALUES ('{shared}', NULL), ('{}', '{shared}'), ('{}', NULL)",
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(Text,), (Text, Nullable<Text>)> =
        sql("SELECT name, email FROM users WHERE name = $1 OR email = $1 ORDER BY name");
    case.assert_same(expected, compile(dsl::parameter_reuse), (&shared,));
}

fn seed_names(case: &mut Case, count: usize) -> Vec<String> {
    let names: Vec<String> = (0..count).map(|_| rnd::text()).collect();
    let rows: Vec<String> = names.iter().map(|n| format!("('{n}')")).collect();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES {}",
        rows.join(", ")
    ));
    names
}

fn unique_id() -> i64 {
    900_000_000 + (rnd::u64() % 100_000_000) as i64
}

#[test]
fn limit_offset_bound() {
    let mut case = Case::new();
    seed_names(&mut case, 4);

    let expected: Query<(Int8, Int8), (Text,)> =
        sql("SELECT name FROM users ORDER BY name LIMIT $1 OFFSET $2");
    case.assert_same(expected, compile(dsl::limit_offset_bound), (2i64, 1i64));
}

#[test]
fn fetch_first_bound() {
    let mut case = Case::new();
    seed_names(&mut case, 3);

    let expected: Query<(Int8,), (Text,)> =
        sql("SELECT name FROM users ORDER BY name FETCH FIRST $1 ROWS ONLY");
    case.assert_same(expected, compile(dsl::fetch_first_bound), (2i64,));
}

#[test]
fn generate_series_bound() {
    let mut case = Case::new();
    seed_names(&mut case, 2);

    let expected: Query<(Int4, Int4), (Text, Int4)> = sql(
        "SELECT u.name, s.day FROM users u CROSS JOIN generate_series($1::int4, $2::int4) AS s(day) ORDER BY u.name, s.day",
    );
    case.assert_same(expected, compile(dsl::generate_series_bound), (2i32, 4i32));
}

#[test]
fn tablesample_bound() {
    let mut case = Case::new();
    seed_names(&mut case, 100);

    let expected: Query<(Float4, Float8), (Text,)> =
        sql("SELECT name FROM users TABLESAMPLE BERNOULLI ($1) REPEATABLE ($2) ORDER BY name");
    case.assert_same(
        expected,
        compile(dsl::tablesample_bound),
        (50.0f32, 42.0f64),
    );
}

#[test]
fn jsonpath_bound() {
    let mut case = Case::new();
    case.exec(r#"INSERT INTO type_samples (v_jsonb) VALUES ('{"a": 1, "b": 2}')"#);

    let expected: Query<(Jsonpath,), (Nullable<Jsonb>,)> =
        sql("SELECT jsonb_path_query(v_jsonb, $1) FROM type_samples");
    case.assert_same(
        expected,
        compile(dsl::jsonpath_bound),
        (Input("$.*".to_string()),),
    );
}

#[test]
fn like_pattern_bound() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('alpha_{}'), ('alpha_{}'), ('beta_{}')",
        rnd::text(),
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(Text,), (Text,)> =
        sql("SELECT name FROM users WHERE name LIKE $1 ORDER BY name");
    case.assert_same(expected, compile(dsl::like_pattern_bound), ("alpha%",));
}

#[test]
fn any_array_bound() {
    let mut case = Case::new();
    let ids = [unique_id(), unique_id(), unique_id()];
    case.exec(&format!(
        "INSERT INTO users (id, name) VALUES ({}, '{}'), ({}, '{}'), ({}, '{}')",
        ids[0],
        rnd::text(),
        ids[1],
        rnd::text(),
        ids[2],
        rnd::text(),
    ));

    let expected: Query<(Array<Int8>,), (Text,)> =
        sql("SELECT name FROM users WHERE id = ANY($1) ORDER BY name");
    case.assert_same(
        expected,
        compile(dsl::any_array_bound),
        (vec![ids[0], ids[2]],),
    );
}

#[test]
fn parameter_reuse_single_slot() {
    let mut case = Case::new();
    let shared = rnd::text();
    case.exec(&format!(
        "INSERT INTO users (name, email) VALUES ('{shared}', NULL), ('{}', '{shared}'), ('{}', NULL)",
        rnd::text(),
        rnd::text(),
    ));

    let expected: Query<(Text,), (Int8,)> =
        sql("SELECT id FROM users WHERE name = $1 OR email = $1 ORDER BY id");
    case.assert_same(
        expected,
        compile(dsl::parameter_reuse_single_slot),
        (shared,),
    );
}

#[test]
fn hostile_text() {
    let mut case = Case::new();
    let hostile = format!("x'); DELETE FROM users; -- $1 {}", rnd::text());
    let escaped = hostile.replace('\'', "''");
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{escaped}'), ('{}')",
        rnd::text()
    ));

    let expected: Query<(Text,), (Text,)> = sql("SELECT name FROM users WHERE name = $1");
    case.assert_same(expected, compile(dsl::hostile_text), (&hostile,));
}
