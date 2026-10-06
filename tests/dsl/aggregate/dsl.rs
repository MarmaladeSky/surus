use crate::support::Query;
use schema::*;
use surus::*;

pub fn count_per_group() -> Query {
    users
        .select((users.group_id, count_star()))
        .group_by(users.group_id)
        .order_by(users.group_id)
        .compile()
        .into()
}

pub fn having() -> Query {
    users
        .select((users.group_id, count_star()))
        .group_by(users.group_id)
        .having(count_star().gt(1))
        .order_by(users.group_id)
        .compile()
        .into()
}

pub fn rollup() -> Query {
    let no_email = users.email.is_null();
    users
        .select((users.group_id, no_email.clone(), count_star()))
        .group_by(surus::rollup((users.group_id, no_email.clone())))
        .order_by((users.group_id, no_email))
        .compile()
        .into()
}

pub fn cube() -> Query {
    let no_email = users.email.is_null();
    users
        .select((users.group_id, no_email.clone(), count_star()))
        .group_by(surus::cube((users.group_id, no_email.clone())))
        .order_by((users.group_id, no_email))
        .compile()
        .into()
}

pub fn grouping_sets() -> Query {
    let no_email = users.email.is_null();
    users
        .select((
            users.group_id,
            no_email.clone(),
            count_star(),
            grouping((users.group_id, no_email.clone())),
        ))
        .group_by(surus::grouping_sets((
            (users.group_id,),
            (no_email.clone(),),
            (),
        )))
        .order_by((users.group_id, no_email))
        .compile()
        .into()
}

pub fn common_aggregates() -> Query {
    products
        .select((
            sum(products.price_cents),
            avg(products.price_cents),
            min(products.name),
            max(products.name),
            bool_or(products.quantity.gt(1)),
        ))
        .compile()
        .into()
}

pub fn filter_clause() -> Query {
    products
        .select((
            count_star().filter(products.quantity.gt(1)),
            sum(products.price_cents).filter(products.quantity.eq(1)),
        ))
        .compile()
        .into()
}

pub fn distinct_inside() -> Query {
    products
        .select((
            count(products.quantity).distinct(),
            array_agg(products.quantity)
                .distinct()
                .order_by(products.quantity),
        ))
        .compile()
        .into()
}

pub fn order_by_inside() -> Query {
    products
        .select((
            string_agg(products.name, ",").order_by(products.price_cents.desc()),
            array_agg(products.price_cents).order_by(products.price_cents),
        ))
        .compile()
        .into()
}

pub fn ordered_set() -> Query {
    products
        .select((
            percentile_cont(0.5).within_group(products.price_cents),
            percentile_disc(0.5).within_group(products.price_cents),
            mode().within_group(products.quantity),
        ))
        .compile()
        .into()
}

pub fn hypothetical_set(price_cents: i32) -> Query {
    products
        .select((
            hypothetical::rank(price_cents).within_group(products.price_cents),
            hypothetical::dense_rank(price_cents).within_group(products.price_cents),
            hypothetical::percent_rank(price_cents).within_group(products.price_cents),
            hypothetical::cume_dist(price_cents).within_group(products.price_cents),
        ))
        .compile()
        .into()
}
