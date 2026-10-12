use crate::schema::{type_samples, users};
use surus::prelude::*;
use surus::types::{Bit, Bool, Date, Int4, Interval, Nullable, Numeric, Text, Time, Timestamp};
use surus::{Query, length, literal, upper};

pub fn quotes_and_backslashes(name: &str) -> Query<(), (Text, Int4)> {
    users
        .filter(|u| u.name.eq(name))
        .select(|u| (u.name, length(u.name)))
        .into()
}

pub fn non_ascii(name: &str) -> Query<(), (Text, Int4, Text)> {
    users
        .filter(|u| u.name.eq(name))
        .select(|u| (u.name, length(u.name), upper(u.name)))
        .into()
}

pub fn typed_literals() -> Query<
    (),
    (
        Nullable<Timestamp>,
        Nullable<Bool>,
        Nullable<Interval>,
        Time,
        Numeric,
    ),
> {
    type_samples
        .select(|s| {
            (
                s.v_date + literal::<Interval>("1 day 2 hours"),
                s.v_date.gt(literal::<Date>("2000-01-01")),
                s.v_timestamp - literal::<Timestamp>("2000-01-01 00:00:00"),
                literal::<Time>("10:30"),
                literal::<Numeric>("12.50"),
            )
        })
        .into()
}

pub fn escape_and_dollar_quoted_strings() -> Query<(), (Text,)> {
    users
        .filter(|u| {
            u.name
                .eq(r#"O'Brien \ "quoted""#)
                .or(u.name.eq("O'Brien \\ \"quoted\""))
                .or(u.name.eq("nested $$ inside"))
        })
        .select(|u| u.name)
        .into()
}

pub fn bit_and_hex_literals() -> Query<(), (Nullable<Bit>, Nullable<Bool>, Int4, Int4)> {
    type_samples
        .select(|s| {
            (
                s.v_bit & literal::<Bit>("11110000"),
                s.v_bit.eq(literal::<Bit>("00001111")),
                literal::<Bit>("11111111").cast::<Int4>(),
                length(literal::<Bit>("1010")),
            )
        })
        .into()
}
