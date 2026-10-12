use crate::schema::{type_samples, users};
use surus::prelude::*;
use surus::types::{Array, Bool, Float4, Float8, Int4, Int8, Jsonb, Jsonpath, Nullable, Text};
use surus::{Query, any_of, from, generate_series, jsonb_path_query, query, select};

pub fn typed_parameter_list() -> Query<(Text, Int4, Int8, Bool), (Text, Int4, Int8, Bool)> {
    query(|(text, int4, int8, bool)| select((text, int4, int8, bool)))
}

pub fn many_parameters_in()
-> Query<(Text, Text, Text, Text, Text, Text, Text, Text, Text, Text), (Text,)> {
    query(|(a, b, c, d, e, f, g, h, i, j)| {
        users
            .filter(|u| u.name.is_in((a, b, c, d, e, f, g, h, i, j)))
            .select(|u| u.name)
            .order_by(|u| u.name)
    })
}

pub fn many_parameters_any() -> Query<(Array<Text>,), (Text,)> {
    query(|(names,)| {
        users
            .filter(|u| u.name.eq(any_of(names)))
            .select(|u| u.name)
            .order_by(|u| u.name)
    })
}

pub fn parameter_reuse() -> Query<(Text,), (Text, Nullable<Text>)> {
    query(|(name,)| {
        users
            .filter(|u| u.name.eq(name).or(u.email.eq(name)))
            .select(|u| (u.name, u.email))
            .order_by(|u| u.name)
    })
}

pub fn limit_offset_bound() -> Query<(Int8, Int8), (Text,)> {
    query(|(limit, offset)| {
        users
            .select(|u| u.name)
            .order_by(|u| u.name)
            .limit(limit)
            .offset(offset)
    })
}

pub fn fetch_first_bound() -> Query<(Int8,), (Text,)> {
    query(|(count,)| {
        users
            .select(|u| u.name)
            .order_by(|u| u.name)
            .fetch_first(count)
    })
}

pub fn generate_series_bound() -> Query<(Int4, Int4), (Text, Int4)> {
    query(|(start, stop)| {
        users
            .cross_join(generate_series(start, stop))
            .select(|(u, (day,))| (u.name, day))
            .order_by(|(u, (day,))| (u.name, day))
    })
}

pub fn tablesample_bound() -> Query<(Float4, Float8), (Text,)> {
    query(|(percent, seed)| {
        from(users.tablesample_bernoulli(percent).repeatable(seed))
            .select(|u| u.name)
            .order_by(|u| u.name)
    })
}

pub fn jsonpath_bound() -> Query<(Jsonpath,), (Nullable<Jsonb>,)> {
    query(|(path,)| type_samples.select(|s| jsonb_path_query(s.v_jsonb, path)))
}

pub fn like_pattern_bound() -> Query<(Text,), (Text,)> {
    query(|(pattern,)| {
        users
            .filter(|u| u.name.like(pattern))
            .select(|u| u.name)
            .order_by(|u| u.name)
    })
}

pub fn any_array_bound() -> Query<(Array<Int8>,), (Text,)> {
    query(|(ids,)| {
        users
            .filter(|u| u.id.eq(any_of(ids)))
            .select(|u| u.name)
            .order_by(|u| u.name)
    })
}

pub fn parameter_reuse_single_slot() -> Query<(Text,), (Int8,)> {
    query(|(name,)| {
        users
            .filter(|u| u.name.eq(name).or(u.email.eq(name)))
            .select(|u| u.id)
            .order_by(|u| u.id)
    })
}

pub fn hostile_text() -> Query<(Text,), (Text,)> {
    query(|(name,)| users.filter(|u| u.name.eq(name)).select(|u| u.name))
}
