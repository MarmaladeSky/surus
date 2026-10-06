use crate::support::Query;
use schema::*;
use surus::*;

pub fn string_functions() -> Query {
    users
        .select((
            upper(users.name),
            length(users.name),
            substr(users.name, 2, 3),
            concat_ws("-", (users.name, users.email)),
            format("%s <%s>", (users.name, users.email)),
        ))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn math_functions() -> Query {
    products
        .select((
            abs(products.price_cents - 500),
            round(products.price_cents / Decimal::new(70, 1), 2),
            power(products.quantity, 2),
            products.price_cents % 7,
        ))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn datetime_functions() -> Query {
    let t = type_samples;
    t.select((
        date_trunc("month", t.v_timestamp),
        extract(DateField::Year, t.v_date),
        to_char(t.v_date, "YYYY-MM-DD"),
    ))
    .compile()
    .into()
}

pub fn named_argument_notation() -> Query {
    products
        .select((
            make_interval().days(products.quantity),
            make_date(2024, 1, param(1) + products.quantity % 28),
        ))
        .order_by(products.name)
        .compile()
        .into()
}

pub fn function_of_function() -> Query {
    users
        .select(upper(substr(lower(users.name), 1, 3)).concat("!"))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn aggregate_of_expression() -> Query {
    users
        .select((
            count_star().filter(users.email.is_not_null()),
            sum(length(users.name) * 2),
        ))
        .compile()
        .into()
}

pub fn window_over_expression() -> Query {
    let running_total =
        sum(products.price_cents * products.quantity).over(window().order_by(products.name));
    products
        .select((products.name, running_total))
        .order_by(products.name)
        .compile()
        .into()
}
