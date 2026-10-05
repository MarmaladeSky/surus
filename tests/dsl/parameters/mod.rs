mod dsl;

use crate::support::{Case, rnd};
use postgres::types::Type;

#[test]
fn typed_parameter_list() {
    let mut case = Case::new();

    case.assert_param_types(&[Type::TEXT, Type::INT4, Type::INT8, Type::BOOL], || {
        dsl::typed_parameter_list("x", 1, 2, true)
    });
}

#[test]
fn many_parameters() {
    let mut case = Case::new();
    let names: Vec<String> = (0..10).map(|_| rnd::text()).collect();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES {}, ('{}')",
        names
            .iter()
            .map(|n| format!("('{n}')"))
            .collect::<Vec<_>>()
            .join(", "),
        rnd::text()
    ));

    case.assert_same(
        &format!(
            "SELECT name FROM users WHERE name IN ({}) ORDER BY name",
            names
                .iter()
                .map(|n| format!("'{n}'"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        || dsl::many_parameters(&names),
    );
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

    case.assert_same(
        &format!("SELECT name, email FROM users WHERE name = '{shared}' OR email = '{shared}' ORDER BY name"),
        || dsl::parameter_reuse(&shared),
    );
}

fn seed_names(case: &mut Case, count: usize) {
    let rows: Vec<String> = (0..count).map(|_| format!("('{}')", rnd::text())).collect();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES {}",
        rows.join(", ")
    ));
}

fn unique_id() -> i64 {
    900_000_000 + (rnd::u64() % 100_000_000) as i64
}

#[test]
fn limit_offset_bound() {
    let mut case = Case::new();
    seed_names(&mut case, 4);

    case.assert_same(
        "SELECT name FROM users ORDER BY name LIMIT 2 OFFSET 1",
        || dsl::limit_offset_bound(2, 1),
    );
    case.assert_param_types(&[Type::INT8, Type::INT8], || dsl::limit_offset_bound(2, 1));
}

#[test]
fn fetch_first_bound() {
    let mut case = Case::new();
    seed_names(&mut case, 3);

    case.assert_same(
        "SELECT name FROM users ORDER BY name FETCH FIRST 2 ROWS ONLY",
        || dsl::fetch_first_bound(2),
    );
    case.assert_param_types(&[Type::INT8], || dsl::fetch_first_bound(2));
}

#[test]
fn generate_series_bound() {
    let mut case = Case::new();
    seed_names(&mut case, 2);

    case.assert_same(
        "SELECT u.name, s.day FROM users u CROSS JOIN generate_series(2, 4) AS s(day) ORDER BY u.name, s.day",
        || dsl::generate_series_bound(2, 4),
    );
    case.assert_param_types(&[Type::INT4, Type::INT4], || {
        dsl::generate_series_bound(2, 4)
    });
}

#[test]
fn tablesample_bound() {
    let mut case = Case::new();
    seed_names(&mut case, 100);

    case.assert_same(
        "SELECT name FROM users TABLESAMPLE BERNOULLI (50) REPEATABLE (42) ORDER BY name",
        || dsl::tablesample_bound(50.0, 42.0),
    );
    case.assert_param_types(&[Type::FLOAT4, Type::FLOAT8], || {
        dsl::tablesample_bound(50.0, 42.0)
    });
}

#[test]
fn jsonpath_bound() {
    let mut case = Case::new();
    case.exec(r#"INSERT INTO type_samples (v_jsonb) VALUES ('{"a": 1, "b": 2}')"#);

    case.assert_same(
        "SELECT jsonb_path_query(v_jsonb, '$.*') FROM type_samples",
        || dsl::jsonpath_bound("$.*"),
    );
    case.assert_param_types(&[Type::JSONPATH], || dsl::jsonpath_bound("$.*"));
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

    case.assert_same(
        "SELECT name FROM users WHERE name LIKE 'alpha%' ORDER BY name",
        || dsl::like_pattern_bound("alpha%"),
    );
    case.assert_param_types(&[Type::TEXT], || dsl::like_pattern_bound("alpha%"));
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

    case.assert_same(
        &format!(
            "SELECT name FROM users WHERE id = ANY(ARRAY[{}, {}]::int8[]) ORDER BY name",
            ids[0], ids[2]
        ),
        || dsl::any_array_bound(&[ids[0], ids[2]]),
    );
    case.assert_param_types(&[Type::INT8_ARRAY], || {
        dsl::any_array_bound(&[ids[0], ids[2]])
    });
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

    case.assert_same(
        &format!("SELECT id FROM users WHERE name = '{shared}' OR email = '{shared}' ORDER BY id"),
        || dsl::parameter_reuse_single_slot(&shared),
    );
    case.assert_param_types(&[Type::TEXT], || dsl::parameter_reuse_single_slot(&shared));
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

    case.assert_same(
        &format!("SELECT name FROM users WHERE name = '{escaped}'"),
        || dsl::hostile_text(&hostile),
    );
}
