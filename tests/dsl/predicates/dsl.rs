use crate::schema::{products, type_samples, users};
use surus::prelude::*;
use surus::types::{Bool, Int4, Int8, Interval, Nullable, Numeric, Text};
use surus::{Query, literal, query, row};

pub fn and_or_not() -> Query<(), (Text,)> {
    users
        .filter(|u| {
            let complete = u.email.is_not_null().and(u.group_id.is_not_null());
            let prefixed = u.name.like("alpha%").or(u.name.like("Beta%"));
            complete.or(prefixed.not())
        })
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}

pub fn is_null() -> Query<(), (Text, Bool, Bool)> {
    users
        .select(|u| (u.name, u.email.is_null(), u.group_id.is_not_null()))
        .order_by(|u| u.name)
        .into()
}

pub fn is_distinct_from() -> Query<(), (Text, Text, Bool)> {
    users
        .join(users, |(a, b)| a.name.lt(b.name))
        .filter(|(a, b)| a.email.is_distinct_from(b.email))
        .select(|(a, b)| (a.name, b.name, a.group_id.is_not_distinct_from(b.group_id)))
        .order_by(|(a, b)| (a.name, b.name))
        .into()
}

pub fn is_true() -> Query<(), (Text, Bool, Bool, Bool)> {
    users
        .select(|u| {
            (
                u.name,
                u.email.is_null().is_true(),
                u.group_id.gt(0).is_not_false(),
                u.group_id.gt(0).is_unknown(),
            )
        })
        .order_by(|u| u.name)
        .into()
}

pub fn between(low: i32, high: i32) -> Query<(), (Text,)> {
    products
        .filter(|p| p.price_cents.between(low, high))
        .select(|p| p.name)
        .order_by(|p| p.name)
        .into()
}

pub fn in_list() -> Query<(Text, Text), (Text,)> {
    query(|(first, second)| {
        users
            .filter(|u| u.name.is_in((first, second)))
            .select(|u| u.name)
            .order_by(|u| u.name)
    })
}

pub fn like(pattern: &str) -> Query<(), (Text,)> {
    users
        .filter(|u| u.name.like(pattern))
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}

pub fn ilike(pattern: &str) -> Query<(), (Text,)> {
    users
        .filter(|u| u.name.ilike(pattern))
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}

pub fn similar_to(pattern: &str) -> Query<(), (Text,)> {
    users
        .filter(|u| u.name.similar_to(pattern))
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}

pub fn like_escape(pattern: &str, escape: &str) -> Query<(), (Text,)> {
    users
        .filter(|u| u.name.like_escape(pattern, escape))
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}

pub fn between_symmetric(first: i32, second: i32) -> Query<(), (Text,)> {
    products
        .filter(|p| p.price_cents.between_symmetric(first, second))
        .select(|p| p.name)
        .order_by(|p| p.name)
        .into()
}

pub fn overlaps() -> Query<(), (Nullable<Bool>, Nullable<Bool>, Nullable<Bool>)> {
    let day = || literal::<Interval>("1 day");
    type_samples
        .select(|s| {
            (
                row((s.v_date, s.v_date + 10)).overlaps(row((s.v_date + 5, s.v_date + 20))),
                row((s.v_date, s.v_date + 10)).overlaps(row((s.v_date + 10, s.v_date + 20))),
                row((s.v_timestamp, day()))
                    .overlaps(row((s.v_timestamp + literal::<Interval>("2 days"), day()))),
            )
        })
        .into()
}

pub fn arithmetic_chain() -> Query<(), (Text, Int4)> {
    products
        .select(|p| (p.name, (p.price_cents + p.quantity) * 2 - 1))
        .order_by(|p| p.name)
        .into()
}

pub fn arithmetic_nullable_column() -> Query<(), (Text, Nullable<Int8>)> {
    users
        .select(|u| (u.name, u.group_id + 1))
        .order_by(|u| u.name)
        .into()
}

pub fn arithmetic_mixed_nullability() -> Query<(), (Text, Nullable<Int8>)> {
    users
        .select(|u| (u.name, u.group_id * u.id))
        .order_by(|u| u.name)
        .into()
}

pub fn negate_expression() -> Query<(), (Text, Int4)> {
    products
        .select(|p| (p.name, -(p.price_cents - p.quantity)))
        .order_by(|p| p.name)
        .into()
}

pub fn compare_expressions() -> Query<(), (Text,)> {
    products
        .filter(|p| (p.price_cents * p.quantity).gt(p.total_cents - 1))
        .select(|p| p.name)
        .order_by(|p| p.name)
        .into()
}

pub fn bool_mixed_nullability() -> Query<(), (Text,)> {
    users
        .filter(|u| {
            u.name
                .ne("")
                .and(u.email.like("v%"))
                .or(u.group_id.is_null())
        })
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}

pub fn operator_results_combined() -> Query<(), (Nullable<Int4>, Nullable<Bool>)> {
    type_samples
        .select(|s| {
            let contained = s.v_int4range.contains(s.v_int4);
            (s.v_int4, contained.and(s.v_box.overlaps(s.v_box)))
        })
        .order_by(|s| s.v_int4)
        .into()
}

pub fn json_chain_into_text_op() -> Query<(), (Nullable<Text>,)> {
    type_samples
        .select(|s| s.v_jsonb.get("a").get_text("b").concat("x"))
        .order_by(|s| s.v_int4)
        .into()
}

pub fn cast_of_expression() -> Query<(), (Text, Numeric)> {
    products
        .select(|p| (p.name, (p.price_cents * p.quantity).cast::<Numeric>() / 100))
        .order_by(|p| p.name)
        .into()
}
