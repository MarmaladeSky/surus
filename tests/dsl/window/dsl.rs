use crate::support::Query;
use schema::*;
use surus::*;

pub fn row_number_by_name() -> Query {
    users
        .select((users.name, row_number().over(window().order_by(users.name))))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn partition_by() -> Query {
    let p = products;
    let per_quantity = window().partition_by(p.quantity);
    p.select((
        p.name,
        p.quantity,
        rank().over(per_quantity.clone().order_by((p.price_cents, p.name))),
        sum(p.price_cents).over(per_quantity.clone()),
        count_star()
            .filter(p.price_cents.gt(500))
            .over(per_quantity),
    ))
    .order_by(p.name)
    .compile()
    .into()
}

pub fn lag_lead() -> Query {
    let p = products;
    let by_price = window().order_by((p.price_cents, p.name));
    p.select((
        p.name,
        lag(p.price_cents).over(by_price.clone()),
        lead(p.price_cents).offset(1).default(0).over(by_price),
    ))
    .order_by((p.price_cents, p.name))
    .compile()
    .into()
}

pub fn frame_clauses() -> Query {
    let p = products;
    let by_price = window().order_by((p.price_cents, p.name));
    p.select((
        p.name,
        sum(p.price_cents).over(by_price.clone().rows_between(preceding(1), current_row())),
        avg(p.price_cents).over(by_price.range_between(unbounded_preceding(), current_row())),
        count_star().over(
            window()
                .order_by(p.quantity)
                .groups_between(current_row(), following(1)),
        ),
    ))
    .order_by((p.price_cents, p.name))
    .compile()
    .into()
}

pub fn named_window() -> Query {
    let p = products;
    let w = surus::named_window(
        "w",
        window()
            .partition_by(p.quantity)
            .order_by((p.price_cents, p.name)),
    );
    p.select((
        p.name,
        rank().over(&w),
        dense_rank().over(&w),
        ntile(2).over(&w),
    ))
    .window(&w)
    .order_by(p.name)
    .compile()
    .into()
}

pub fn value_functions() -> Query {
    let p = products;
    let w = surus::named_window("w", window().order_by((p.price_cents, p.name)));
    p.select((
        p.name,
        first_value(p.name).over(&w),
        last_value(p.name).over(w.rows_between(unbounded_preceding(), unbounded_following())),
        nth_value(p.name, 2).over(&w),
        percent_rank().over(&w),
        cume_dist().over(&w),
    ))
    .window(&w)
    .order_by((p.price_cents, p.name))
    .compile()
    .into()
}

pub fn frame_exclusion_and_offsets() -> Query {
    let p = products;
    let by_quantity = window().order_by(p.quantity);
    p.select((
        p.name,
        sum(p.price_cents).over(
            window()
                .order_by((p.price_cents, p.name))
                .rows_between(preceding(1), following(1))
                .exclude_current_row(),
        ),
        sum(p.price_cents).over(
            window()
                .order_by(p.price_cents)
                .range_between(preceding(100), following(100)),
        ),
        count_star().over(
            by_quantity
                .clone()
                .range_between(current_row(), unbounded_following())
                .exclude_ties(),
        ),
        count_star().over(
            by_quantity
                .rows_between(unbounded_preceding(), unbounded_following())
                .exclude_group(),
        ),
    ))
    .order_by((p.price_cents, p.name))
    .compile()
    .into()
}
