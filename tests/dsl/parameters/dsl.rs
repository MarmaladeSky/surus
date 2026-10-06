use crate::support::Query;
use schema::*;
use surus::*;

pub fn typed_parameter_list(name: &str, price_cents: i32, id: i64, flag: bool) -> Query {
    let p = products;
    p.select(p.name)
        .filter(
            p.name
                .eq(name)
                .and(p.price_cents.eq(price_cents))
                .and(p.id.eq(id))
                .and(flag),
        )
        .compile()
        .into()
}

pub fn many_parameters(names: &[String]) -> Query {
    users
        .select(users.name)
        .filter(users.name.in_(names))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn parameter_reuse(value: &str) -> Query {
    let value = param(value);
    users
        .select((users.name, users.email))
        .filter(users.name.eq(value.clone()).or(users.email.eq(value)))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn limit_offset_bound(limit: i64, offset: i64) -> Query {
    users
        .select(users.name)
        .order_by(users.name)
        .limit(limit)
        .offset(offset)
        .compile()
        .into()
}

pub fn fetch_first_bound(count: i64) -> Query {
    users
        .select(users.name)
        .order_by(users.name)
        .fetch_first(count)
        .compile()
        .into()
}

pub fn generate_series_bound(from: i32, to: i32) -> Query {
    let u = alias!(users, "u");
    let s = alias!(generate_series(from, to), "s");
    let (day,) = s.columns();
    u.cross_join(s)
        .select((u.name, day))
        .order_by((u.name, day))
        .compile()
        .into()
}

pub fn tablesample_bound(percent: f32, seed: f64) -> Query {
    users
        .tablesample_bernoulli(percent)
        .repeatable(seed)
        .select(users.name)
        .order_by(users.name)
        .compile()
        .into()
}

pub fn jsonpath_bound(path: &str) -> Query {
    type_samples
        .select(jsonb_path_query(type_samples.v_jsonb, path))
        .compile()
        .into()
}

pub fn like_pattern_bound(pattern: &str) -> Query {
    users
        .select(users.name)
        .filter(users.name.like(pattern))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn any_array_bound(ids: &[i64]) -> Query {
    users
        .select(users.name)
        .filter(users.id.eq(any(ids)))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn parameter_reuse_single_slot(value: &str) -> Query {
    let value = param(value);
    users
        .select(users.id)
        .filter(users.name.eq(value.clone()).or(users.email.eq(value)))
        .order_by(users.id)
        .compile()
        .into()
}

pub fn hostile_text(name: &str) -> Query {
    users
        .select(users.name)
        .filter(users.name.eq(name))
        .compile()
        .into()
}
