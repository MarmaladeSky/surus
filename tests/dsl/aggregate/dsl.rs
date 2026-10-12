use crate::schema::{products, users};
use surus::prelude::*;
use surus::types::{Array, Bool, Float8, Int4, Int8, Nullable, Numeric, Text};
use surus::{
    Query, array_agg, avg, bool_or, count, count_star, grouping, hypothetical_cume_dist,
    hypothetical_dense_rank, hypothetical_percent_rank, hypothetical_rank, max, min, mode,
    percentile_cont, percentile_disc, string_agg, sum,
};

pub fn count_per_group() -> Query<(), (Nullable<Int8>, Int8)> {
    users
        .group_by(|u| u.group_id)
        .select(|u| (u.group_id, count_star()))
        .order_by(|u| u.group_id)
        .into()
}

pub fn having() -> Query<(), (Nullable<Int8>, Int8)> {
    users
        .group_by(|u| u.group_id)
        .having(|_| count_star().gt(1))
        .select(|u| (u.group_id, count_star()))
        .order_by(|u| u.group_id)
        .into()
}

pub fn rollup() -> Query<(), (Nullable<Int8>, Nullable<Bool>, Int8)> {
    users
        .group_by(|u| surus::rollup((u.group_id, u.email.is_null())))
        .select(|(_, (group_id, no_email))| (group_id, no_email, count_star()))
        .order_by(|(_, keys)| keys)
        .into()
}

pub fn cube() -> Query<(), (Nullable<Int8>, Nullable<Bool>, Int8)> {
    users
        .group_by(|u| surus::cube((u.group_id, u.email.is_null())))
        .select(|(_, (group_id, no_email))| (group_id, no_email, count_star()))
        .order_by(|(_, keys)| keys)
        .into()
}

pub fn grouping_sets() -> Query<(), (Nullable<Int8>, Nullable<Bool>, Int8, Int4)> {
    users
        .group_by(|u| {
            surus::grouping_sets((u.group_id, u.email.is_null()), |(group_id, no_email)| {
                ((group_id,), (no_email,), ())
            })
        })
        .select(|(_, (group_id, no_email))| {
            (
                group_id,
                no_email,
                count_star(),
                grouping((group_id, no_email)),
            )
        })
        .order_by(|(_, keys)| keys)
        .into()
}

pub fn common_aggregates() -> Query<
    (),
    (
        Nullable<Int8>,
        Nullable<Numeric>,
        Nullable<Text>,
        Nullable<Text>,
        Nullable<Bool>,
    ),
> {
    products
        .select(|p| {
            (
                sum(p.price_cents),
                avg(p.price_cents),
                min(p.name),
                max(p.name),
                bool_or(p.quantity.gt(1)),
            )
        })
        .into()
}

pub fn filter_clause() -> Query<(), (Int8, Nullable<Int8>)> {
    products
        .select(|p| {
            (
                count_star().filter(p.quantity.gt(1)),
                sum(p.price_cents).filter(p.quantity.eq(1)),
            )
        })
        .into()
}

pub fn distinct_inside() -> Query<(), (Int8, Nullable<Array<Int4>>)> {
    products
        .select(|p| {
            (
                count(p.quantity).distinct(),
                array_agg(p.quantity).distinct().order_by(p.quantity),
            )
        })
        .into()
}

pub fn order_by_inside() -> Query<(), (Nullable<Text>, Nullable<Array<Int4>>)> {
    products
        .select(|p| {
            (
                string_agg(p.name, ",").order_by(p.price_cents.desc()),
                array_agg(p.price_cents).order_by(p.price_cents),
            )
        })
        .into()
}

pub fn ordered_set() -> Query<(), (Nullable<Float8>, Nullable<Int4>, Nullable<Int4>)> {
    products
        .select(|p| {
            (
                percentile_cont(0.5).within_group(p.price_cents),
                percentile_disc(0.5).within_group(p.price_cents),
                mode().within_group(p.quantity),
            )
        })
        .into()
}

pub fn hypothetical_set(price_cents: i32) -> Query<(), (Int8, Int8, Float8, Float8)> {
    products
        .select(|p| {
            (
                hypothetical_rank(price_cents).within_group(p.price_cents),
                hypothetical_dense_rank(price_cents).within_group(p.price_cents),
                hypothetical_percent_rank(price_cents).within_group(p.price_cents),
                hypothetical_cume_dist(price_cents).within_group(p.price_cents),
            )
        })
        .into()
}
