use crate::support::Query;
use schema::*;
use std::time::Duration;
use surus::*;

pub fn quotes_and_backslashes(name: &str) -> Query {
    users
        .select((users.name, length(users.name)))
        .filter(users.name.eq(name))
        .compile()
        .into()
}

pub fn non_ascii(name: &str) -> Query {
    users
        .select((users.name, length(users.name), upper(users.name)))
        .filter(users.name.eq(name))
        .compile()
        .into()
}

pub fn typed_literals() -> Query {
    let t = type_samples;
    let one_day_two_hours = Duration::from_secs((24 + 2) * 60 * 60);
    let new_year_2000 = make_date(2000, 1, 1);
    t.select((
        t.v_date + one_day_two_hours,
        t.v_date.gt(new_year_2000.clone()),
        t.v_timestamp - (new_year_2000 + make_time(0, 0, 0.0)),
        make_time(10, 30, 0.0),
        Decimal::new(1250, 2),
    ))
    .compile()
    .into()
}

pub fn escape_and_dollar_quoted_strings() -> Query {
    let tricky = "O'Brien \\ \"quoted\"";
    users
        .select(users.name)
        .filter(
            users
                .name
                .eq(tricky)
                .or(users.name.eq(tricky))
                .or(users.name.eq("nested $$ inside")),
        )
        .compile()
        .into()
}

pub fn bit_and_hex_literals() -> Query {
    let t = type_samples;
    t.select((
        t.v_bit & Bits::new(0b1111_0000, 8),
        t.v_bit.eq(Bits::new(0x0F, 8)),
        param(Bits::new(0xFF, 8)).cast::<Int4>(),
        length(Bits::new(0b1010, 4)),
    ))
    .compile()
    .into()
}
