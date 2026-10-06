use crate::support::Query;
use schema::*;
use surus::*;

pub fn jsonb_build_object() -> Query {
    let object = surus::jsonb_build_object(("name", products.name, "price", products.price_cents));
    products
        .select(object)
        .order_by(products.name)
        .compile()
        .into()
}

pub fn jsonb_set() -> Query {
    let t = type_samples;
    t.select((surus::jsonb_set(t.v_jsonb, ["k"], "42"), t.v_jsonb - "s"))
        .compile()
        .into()
}

pub fn sql_json_constructors() -> Query {
    products
        .select((
            json_object((("name", products.name), ("qty", products.quantity))),
            json_array((products.price_cents, products.quantity)),
        ))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn json_table() -> Query {
    let columns = (
        json_column::<Int4>("k", "$.k"),
        json_column::<Text>("s", "$.s"),
    );
    let jt = alias!(surus::json_table(type_samples.v_jsonb, "$", columns), "jt");
    let (k, s) = jt.columns();
    type_samples.cross_join(jt).select((k, s)).compile().into()
}

pub fn is_json_predicates() -> Query {
    let t = type_samples;
    t.select((
        t.v_text.is_json(),
        t.v_jsonb.is_json_object(),
        t.v_jsonb.is_json_array(),
        t.v_jsonb.is_json_scalar(),
        t.v_jsonb.cast::<Text>().is_json_with_unique_keys(),
        t.v_text.is_not_json(),
    ))
    .compile()
    .into()
}

pub fn json_exists_query_value() -> Query {
    let t = type_samples;
    t.select((
        json_exists(t.v_jsonb, "$.k"),
        json_query(t.v_jsonb, "$.s").with_wrapper(),
        json_value(t.v_jsonb, "$.k").returning::<Int4>(),
        json_value(t.v_jsonb, "$.missing").default_on_empty("-1"),
        json_value(t.v_jsonb, "$.k.x").default_on_error("err"),
        json_query(t.v_jsonb, "$.nope").empty_array_on_empty(),
    ))
    .compile()
    .into()
}

pub fn json_serialize_and_constructors() -> Query {
    let t = type_samples;
    t.select((
        json_serialize(t.v_jsonb),
        json_serialize(t.v_jsonb).returning::<Bytea>(),
        json_scalar(t.v_int4),
        json_scalar(t.v_text),
        json(t.v_jsonb.cast::<Text>()),
    ))
    .compile()
    .into()
}

pub fn jsonb_aggregates() -> Query {
    products
        .select((
            jsonb_agg(products.name).order_by(products.name),
            jsonb_object_agg(products.name, products.price_cents),
            json_agg(products.quantity).order_by(products.name),
        ))
        .compile()
        .into()
}

pub fn jsonb_path_functions() -> Query {
    let t = type_samples;
    let items = jsonb_path_query(t.v_jsonb, "$.*");
    let (item,) = items.columns();
    t.cross_join(items)
        .select((
            item,
            jsonb_path_query_array(t.v_jsonb, "$.*"),
            jsonb_path_exists(t.v_jsonb, "$.k ? (@ > $min)", r#"{"min": 0}"#),
        ))
        .order_by(item.cast::<Text>())
        .compile()
        .into()
}
