mod dsl;

use crate::support::{Case, compile, literal, rnd, sql};
use surus::Query;
use surus::types::{Bit, Bool, Int4, Interval, Nullable, Numeric, Text, Time, Timestamp};

const TRICKY: &str = "O'Brien \\ \"quoted\"";
const UNICODE: &str = "Zoë – 日本語 🚀";

#[test]
fn quotes_and_backslashes() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('O''Brien \\ \"quoted\"'), ('{}')",
        rnd::text()
    ));

    let expected: Query<(), (Text, Int4)> =
        sql("SELECT name, length(name) FROM users WHERE name = 'O''Brien \\ \"quoted\"'");
    case.assert_same(
        expected,
        compile(|| dsl::quotes_and_backslashes(TRICKY)),
        (),
    );
}

#[test]
fn non_ascii() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{UNICODE}'), ('{}')",
        rnd::text()
    ));

    let expected: Query<(), (Text, Int4, Text)> = sql(&format!(
        "SELECT name, length(name), upper(name) FROM users WHERE name = '{UNICODE}'"
    ));
    case.assert_same(expected, compile(|| dsl::non_ascii(UNICODE)), ());
}

#[test]
fn typed_literals() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_date, v_timestamp) VALUES ('{}', '{}')",
        literal::date(),
        literal::timestamp(),
    ));

    let expected: Query<
        (),
        (
            Nullable<Timestamp>,
            Nullable<Bool>,
            Nullable<Interval>,
            Time,
            Numeric,
        ),
    > = sql("SELECT v_date + INTERVAL '1 day 2 hours',
                v_date > DATE '2000-01-01',
                v_timestamp - TIMESTAMP '2000-01-01 00:00:00',
                TIME '10:30',
                NUMERIC '12.50'
         FROM type_samples");
    case.assert_same(expected, compile(dsl::typed_literals), ());
}

#[test]
fn escape_and_dollar_quoted_strings() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('O''Brien \\ \"quoted\"'), ('{}')",
        rnd::text()
    ));

    let expected: Query<(), (Text,)> = sql("SELECT name FROM users
         WHERE name = $$O'Brien \\ \"quoted\"$$
            OR name = E'O\\'Brien \\\\ \"quoted\"'
            OR name = $tag$nested $$ inside$tag$");
    case.assert_same(expected, compile(dsl::escape_and_dollar_quoted_strings), ());
}

#[test]
fn bit_and_hex_literals() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_bit) VALUES ('{}')",
        literal::bit()
    ));

    let expected: Query<(), (Nullable<Bit>, Nullable<Bool>, Int4, Int4)> = sql(
        "SELECT v_bit & B'11110000', v_bit = X'0F', X'FF'::int4, length(B'1010') FROM type_samples",
    );
    case.assert_same(expected, compile(dsl::bit_and_hex_literals), ());
}
