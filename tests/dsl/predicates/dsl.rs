use crate::support::Query;
use schema::*;
use std::time::Duration;
use surus::*;

pub fn and_or_not() -> Query {
    let complete = users.email.is_not_null().and(users.group_id.is_not_null());
    let alpha_or_beta = users.name.like("alpha%").or(users.name.like("Beta%"));
    users
        .select(users.name)
        .filter(complete.or(!alpha_or_beta))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn is_null() -> Query {
    users
        .select((
            users.name,
            users.email.is_null(),
            users.group_id.is_not_null(),
        ))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn is_distinct_from() -> Query {
    let (a, b) = (alias!(users, "a"), alias!(users, "b"));
    a.join(b, a.name.lt(b.name))
        .select((a.name, b.name, a.group_id.is_not_distinct_from(b.group_id)))
        .filter(a.email.is_distinct_from(b.email))
        .order_by((a.name, b.name))
        .compile()
        .into()
}

pub fn is_true() -> Query {
    let has_group = users.group_id.gt(0);
    users
        .select((
            users.name,
            users.email.is_null().is_true(),
            has_group.clone().is_not_false(),
            has_group.is_unknown(),
        ))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn between(low: i32, high: i32) -> Query {
    products
        .select(products.name)
        .filter(products.price_cents.between(low, high))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn in_list(first: &str, second: &str) -> Query {
    users
        .select(users.name)
        .filter(users.name.in_([first, second]))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn like(pattern: &str) -> Query {
    users
        .select(users.name)
        .filter(users.name.like(pattern))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn ilike(pattern: &str) -> Query {
    users
        .select(users.name)
        .filter(users.name.ilike(pattern))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn similar_to(pattern: &str) -> Query {
    users
        .select(users.name)
        .filter(users.name.similar_to(pattern))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn like_escape(pattern: &str, escape: &str) -> Query {
    users
        .select(users.name)
        .filter(users.name.like_escape(pattern, escape))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn between_symmetric(first: i32, second: i32) -> Query {
    products
        .select(products.name)
        .filter(products.price_cents.between_symmetric(first, second))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn overlaps() -> Query {
    let (date, timestamp) = (type_samples.v_date, type_samples.v_timestamp);
    let day = Duration::from_secs(24 * 60 * 60);
    type_samples
        .select((
            period(date, date + 10).overlaps(period(date + 5, date + 20)),
            period(date, date + 10).overlaps(period(date + 10, date + 20)),
            period(timestamp, day).overlaps(period(timestamp + day * 2, day)),
        ))
        .compile()
        .into()
}

pub fn arithmetic_chain() -> Query {
    products
        .select((
            products.name,
            (products.price_cents + products.quantity) * 2 - 1,
        ))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn arithmetic_nullable_column() -> Query {
    users
        .select((users.name, users.group_id + 1))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn arithmetic_mixed_nullability() -> Query {
    users
        .select((users.name, users.group_id * users.id))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn negate_expression() -> Query {
    products
        .select((products.name, -(products.price_cents - products.quantity)))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn compare_expressions() -> Query {
    products
        .select(products.name)
        .filter((products.price_cents * products.quantity).gt(products.total_cents - 1))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn bool_mixed_nullability() -> Query {
    users
        .select(users.name)
        .filter(
            users
                .name
                .ne("")
                .and(users.email.like("v%"))
                .or(users.group_id.is_null()),
        )
        .order_by(users.name)
        .compile()
        .into()
}

pub fn operator_results_combined() -> Query {
    let t = type_samples;
    t.select((
        t.v_int4,
        t.v_int4range
            .contains(t.v_int4)
            .and(t.v_box.overlaps(t.v_box)),
    ))
    .order_by(t.v_int4)
    .compile()
    .into()
}

pub fn json_chain_into_text_op() -> Query {
    let t = type_samples;
    t.select(t.v_jsonb.get("a").get_text("b").concat("x"))
        .order_by(t.v_int4)
        .compile()
        .into()
}

pub fn cast_of_expression() -> Query {
    products
        .select((
            products.name,
            (products.price_cents * products.quantity).cast::<Numeric>() / 100,
        ))
        .order_by(products.name)
        .compile()
        .into()
}
