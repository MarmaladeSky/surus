use crate::schema::{products, type_samples};
use surus::prelude::*;
use surus::types::{Bool, Bytea, Int4, Json, Jsonb, Nullable, Text};
use surus::{
    JsonPredicates, Query, json, json_agg, json_array, json_column, json_exists, json_object,
    json_query, json_scalar, json_serialize, json_serialize_as, json_value, jsonb_agg,
    jsonb_object_agg, jsonb_path_exists, jsonb_path_query, jsonb_path_query_array, jsonpath,
    literal,
};

pub fn jsonb_build_object() -> Query<(), (Jsonb,)> {
    products
        .select(|p| surus::jsonb_build_object((("name", p.name), ("price", p.price_cents))))
        .order_by(|p| p.name)
        .into()
}

pub fn jsonb_set() -> Query<(), (Nullable<Jsonb>, Nullable<Jsonb>)> {
    type_samples
        .select(|s| {
            (
                surus::jsonb_set(s.v_jsonb, vec!["k"], literal::<Jsonb>("42")),
                s.v_jsonb.remove("s"),
            )
        })
        .into()
}

pub fn sql_json_constructors() -> Query<(), (Json, Json)> {
    products
        .select(|p| {
            (
                json_object((("name", p.name), ("qty", p.quantity))),
                json_array((p.price_cents, p.quantity)),
            )
        })
        .order_by(|p| p.name)
        .into()
}

pub fn json_table() -> Query<(), (Nullable<Int4>, Nullable<Text>)> {
    type_samples
        .cross_join_lateral(|s| {
            let columns = (json_column::<Int4>("$.k"), json_column::<Text>("$.s"));
            surus::json_table(s.v_jsonb, "$", columns)
        })
        .select(|(_, (k, s))| (k, s))
        .into()
}

pub fn is_json_predicates() -> Query<
    (),
    (
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
    ),
> {
    type_samples
        .select(|s| {
            (
                s.v_text.is_json(),
                s.v_jsonb.is_json_object(),
                s.v_jsonb.is_json_array(),
                s.v_jsonb.is_json_scalar(),
                s.v_jsonb.cast::<Text>().is_json_with_unique_keys(),
                s.v_text.is_not_json(),
            )
        })
        .into()
}

pub fn json_exists_query_value() -> Query<
    (),
    (
        Nullable<Bool>,
        Nullable<Jsonb>,
        Nullable<Int4>,
        Nullable<Text>,
        Nullable<Text>,
        Nullable<Jsonb>,
    ),
> {
    type_samples
        .select(|s| {
            let doc = s.v_jsonb;
            (
                json_exists(doc, jsonpath("$.k")),
                json_query(doc, jsonpath("$.s")).with_wrapper(),
                json_value(doc, jsonpath("$.k")).returning::<Int4>(),
                json_value(doc, jsonpath("$.missing")).default_on_empty("-1"),
                json_value(doc, jsonpath("$.k.x")).default_on_error("err"),
                json_query(doc, jsonpath("$.nope")).empty_array_on_empty(),
            )
        })
        .into()
}

pub fn json_serialize_and_constructors() -> Query<
    (),
    (
        Nullable<Text>,
        Nullable<Bytea>,
        Nullable<Json>,
        Nullable<Json>,
        Nullable<Json>,
    ),
> {
    type_samples
        .select(|s| {
            (
                json_serialize(s.v_jsonb),
                json_serialize_as::<Bytea, _>(s.v_jsonb),
                json_scalar(s.v_int4),
                json_scalar(s.v_text),
                json(s.v_jsonb.cast::<Text>()),
            )
        })
        .into()
}

pub fn jsonb_aggregates() -> Query<(), (Nullable<Jsonb>, Nullable<Jsonb>, Nullable<Json>)> {
    products
        .select(|p| {
            (
                jsonb_agg(p.name).order_by(p.name),
                jsonb_object_agg(p.name, p.price_cents),
                json_agg(p.quantity).order_by(p.name),
            )
        })
        .into()
}

pub fn jsonb_path_functions() -> Query<(), (Nullable<Jsonb>, Nullable<Jsonb>, Nullable<Bool>)> {
    type_samples
        .cross_join_lateral(|s| jsonb_path_query(s.v_jsonb, jsonpath("$.*")))
        .select(|(s, (p,))| {
            (
                p,
                jsonb_path_query_array(s.v_jsonb, jsonpath("$.*")),
                jsonb_path_exists(
                    s.v_jsonb,
                    jsonpath("$.k ? (@ > $min)"),
                    literal::<Jsonb>(r#"{"min": 0}"#),
                ),
            )
        })
        .order_by(|(_, (p,))| p.cast::<Text>())
        .into()
}
