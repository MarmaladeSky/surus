mod dsl;

use crate::support::{Case, literal, rnd};

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

fn seed_sample(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO type_samples (v_jsonb) VALUES ('{}')",
        literal::jsonb()
    ));
}

#[test]
fn jsonb_build_object() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT jsonb_build_object('name', name, 'price', price_cents) FROM products ORDER BY name",
        dsl::jsonb_build_object,
    );
}

#[test]
fn jsonb_set() {
    let mut case = Case::new();
    seed_sample(&mut case);

    case.assert_same(
        "SELECT jsonb_set(v_jsonb, '{k}', '42'), v_jsonb - 's' FROM type_samples",
        dsl::jsonb_set,
    );
}

#[test]
fn sql_json_constructors() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT JSON_OBJECT('name': name, 'qty': quantity), JSON_ARRAY(price_cents, quantity) FROM products ORDER BY name",
        dsl::sql_json_constructors,
    );
}

#[test]
fn json_table() {
    let mut case = Case::new();
    seed_sample(&mut case);

    case.assert_same(
        "SELECT jt.k, jt.s
         FROM type_samples,
              JSON_TABLE(v_jsonb, '$' COLUMNS (k int4 PATH '$.k', s text PATH '$.s')) AS jt",
        dsl::json_table,
    );
}

fn seed_sample_with_scalars(case: &mut Case) {
    case.exec(&format!(
        "INSERT INTO type_samples (v_jsonb, v_text, v_int4) VALUES ('{}', '{}', {})",
        literal::jsonb(),
        literal::text(),
        literal::int4(),
    ));
}

#[test]
fn is_json_predicates() {
    let mut case = Case::new();
    seed_sample_with_scalars(&mut case);

    case.assert_same(
        "SELECT v_text IS JSON, v_jsonb IS JSON OBJECT, v_jsonb IS JSON ARRAY, v_jsonb IS JSON SCALAR,
                v_jsonb::text IS JSON WITH UNIQUE KEYS, v_text IS NOT JSON
         FROM type_samples",
        dsl::is_json_predicates,
    );
}

#[test]
fn json_exists_query_value() {
    let mut case = Case::new();
    seed_sample(&mut case);

    case.assert_same(
        "SELECT JSON_EXISTS(v_jsonb, '$.k'),
                JSON_QUERY(v_jsonb, '$.s' WITH WRAPPER),
                JSON_VALUE(v_jsonb, '$.k' RETURNING int4),
                JSON_VALUE(v_jsonb, '$.missing' DEFAULT -1 ON EMPTY),
                JSON_VALUE(v_jsonb, '$.k.x' DEFAULT 'err' ON ERROR),
                JSON_QUERY(v_jsonb, '$.nope' EMPTY ARRAY ON EMPTY)
         FROM type_samples",
        dsl::json_exists_query_value,
    );
}

#[test]
fn json_serialize_and_constructors() {
    let mut case = Case::new();
    seed_sample_with_scalars(&mut case);

    case.assert_same(
        "SELECT JSON_SERIALIZE(v_jsonb), JSON_SERIALIZE(v_jsonb RETURNING bytea),
                JSON_SCALAR(v_int4), JSON_SCALAR(v_text), JSON(v_jsonb::text)
         FROM type_samples",
        dsl::json_serialize_and_constructors,
    );
}

#[test]
fn jsonb_aggregates() {
    let mut case = Case::new();
    seed_products(&mut case);

    case.assert_same(
        "SELECT jsonb_agg(name ORDER BY name), jsonb_object_agg(name, price_cents), json_agg(quantity ORDER BY name)
         FROM products",
        dsl::jsonb_aggregates,
    );
}

#[test]
fn jsonb_path_functions() {
    let mut case = Case::new();
    seed_sample(&mut case);

    case.assert_same(
        "SELECT p, jsonb_path_query_array(v_jsonb, '$.*'), jsonb_path_exists(v_jsonb, '$.k ? (@ > $min)', '{\"min\": 0}')
         FROM type_samples, jsonb_path_query(v_jsonb, '$.*') AS p
         ORDER BY p::text",
        dsl::jsonb_path_functions,
    );
}
