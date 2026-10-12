use crate::schema::{products, users};
use surus::prelude::*;
use surus::types::{Float8, Int4, Int8, Nullable, Numeric, Text};
use surus::{
    Query, avg, count_star, cume_dist, current_row, dense_rank, first_value, following, lag,
    last_value, lead, nth_value, ntile, percent_rank, preceding, rank, row_number, sum,
    unbounded_following, unbounded_preceding, window,
};

pub fn row_number_by_name() -> Query<(), (Text, Int8)> {
    users
        .select(|u| (u.name, row_number().over(window().order_by(u.name))))
        .order_by(|u| u.name)
        .into()
}

pub fn partition_by() -> Query<(), (Text, Int4, Int8, Nullable<Int8>, Int8)> {
    products
        .select(|p| {
            let by_quantity = window().partition_by(p.quantity);
            (
                p.name,
                p.quantity,
                rank().over(by_quantity.clone().order_by((p.price_cents, p.name))),
                sum(p.price_cents).over(by_quantity.clone()),
                count_star().filter(p.price_cents.gt(500)).over(by_quantity),
            )
        })
        .order_by(|p| p.name)
        .into()
}

pub fn lag_lead() -> Query<(), (Text, Nullable<Int4>, Nullable<Int4>)> {
    products
        .select(|p| {
            let by_price = window().order_by((p.price_cents, p.name));
            (
                p.name,
                lag(p.price_cents).over(by_price.clone()),
                lead(p.price_cents, 1, 0).over(by_price),
            )
        })
        .order_by(|p| (p.price_cents, p.name))
        .into()
}

pub fn frame_clauses() -> Query<(), (Text, Nullable<Int8>, Nullable<Numeric>, Int8)> {
    products
        .select(|p| {
            let by_price = window().order_by((p.price_cents, p.name));
            (
                p.name,
                sum(p.price_cents).over(by_price.clone().rows(preceding(1), current_row())),
                avg(p.price_cents).over(by_price.range(unbounded_preceding(), current_row())),
                count_star().over(
                    window()
                        .order_by(p.quantity)
                        .groups(current_row(), following(1)),
                ),
            )
        })
        .order_by(|p| (p.price_cents, p.name))
        .into()
}

pub fn named_window() -> Query<(), (Text, Int8, Int8, Int4)> {
    products
        .select(|p| {
            let w = window()
                .partition_by(p.quantity)
                .order_by((p.price_cents, p.name));
            (
                p.name,
                rank().over(w.clone()),
                dense_rank().over(w.clone()),
                ntile(2).over(w),
            )
        })
        .order_by(|p| p.name)
        .into()
}

pub fn value_functions() -> Query<
    (),
    (
        Text,
        Nullable<Text>,
        Nullable<Text>,
        Nullable<Text>,
        Float8,
        Float8,
    ),
> {
    products
        .select(|p| {
            let w = window().order_by((p.price_cents, p.name));
            let whole = w.clone().rows(unbounded_preceding(), unbounded_following());
            (
                p.name,
                first_value(p.name).over(w.clone()),
                last_value(p.name).over(whole),
                nth_value(p.name, 2).over(w.clone()),
                percent_rank().over(w.clone()),
                cume_dist().over(w),
            )
        })
        .order_by(|p| (p.price_cents, p.name))
        .into()
}

pub fn frame_exclusion_and_offsets() -> Query<(), (Text, Nullable<Int8>, Nullable<Int8>, Int8, Int8)>
{
    products
        .select(|p| {
            let by_quantity = window().order_by(p.quantity);
            (
                p.name,
                sum(p.price_cents).over(
                    window()
                        .order_by((p.price_cents, p.name))
                        .rows(preceding(1), following(1))
                        .exclude_current_row(),
                ),
                sum(p.price_cents).over(
                    window()
                        .order_by(p.price_cents)
                        .range(preceding(100), following(100)),
                ),
                count_star().over(
                    by_quantity
                        .clone()
                        .range(current_row(), unbounded_following())
                        .exclude_ties(),
                ),
                count_star().over(
                    by_quantity
                        .rows(unbounded_preceding(), unbounded_following())
                        .exclude_group(),
                ),
            )
        })
        .order_by(|p| (p.price_cents, p.name))
        .into()
}
