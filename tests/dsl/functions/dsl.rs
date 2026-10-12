use crate::schema::{products, type_samples, users};
use surus::prelude::*;
use surus::types::{Date, Float8, Int4, Int8, Interval, Nullable, Numeric, Text, Timestamp};
use surus::{
    DatePart, Query, abs, concat_ws, count_star, date_trunc, extract, format, length, literal,
    lower, make_date, make_interval, power, round, substr, sum, to_char, upper, window,
};

pub fn string_functions() -> Query<(), (Text, Int4, Text, Text, Text)> {
    users
        .select(|u| {
            (
                upper(u.name),
                length(u.name),
                substr(u.name, 2, 3),
                concat_ws("-", (u.name, u.email)),
                format("%s <%s>", (u.name, u.email)),
            )
        })
        .order_by(|u| u.name)
        .into()
}

pub fn math_functions() -> Query<(), (Int4, Numeric, Float8, Int4)> {
    products
        .select(|p| {
            (
                abs(p.price_cents - 500),
                round(p.price_cents / literal::<Numeric>("7.0"), 2),
                power(p.quantity, 2),
                p.price_cents % 7,
            )
        })
        .order_by(|p| p.name)
        .into()
}

pub fn datetime_functions() -> Query<(), (Nullable<Timestamp>, Nullable<Numeric>, Nullable<Text>)> {
    type_samples
        .select(|s| {
            (
                date_trunc("month", s.v_timestamp),
                extract(DatePart::Year, s.v_date),
                to_char(s.v_date, "YYYY-MM-DD"),
            )
        })
        .into()
}

pub fn named_argument_notation() -> Query<(), (Interval, Date)> {
    products
        .select(|p| {
            (
                make_interval().days(p.quantity),
                make_date(2024, 1, p.quantity % 28 + 1),
            )
        })
        .order_by(|p| p.name)
        .into()
}

pub fn function_of_function() -> Query<(), (Text,)> {
    users
        .select(|u| upper(substr(lower(u.name), 1, 3)).concat("!"))
        .order_by(|u| u.name)
        .into()
}

pub fn aggregate_of_expression() -> Query<(), (Int8, Nullable<Int8>)> {
    users
        .select(|u| {
            (
                count_star().filter(u.email.is_not_null()),
                sum(length(u.name) * 2),
            )
        })
        .into()
}

pub fn window_over_expression() -> Query<(), (Text, Nullable<Int8>)> {
    products
        .select(|p| {
            let running = sum(p.price_cents * p.quantity).over(window().order_by(p.name));
            (p.name, running)
        })
        .order_by(|p| p.name)
        .into()
}
