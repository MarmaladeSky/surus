mod dsl;

use crate::support::{Case, literal, rnd};

const TRICKY: &str = "O'Brien \\ \"quoted\"";
const UNICODE: &str = "Zoë – 日本語 🚀";

#[test]
fn quotes_and_backslashes() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('O''Brien \\ \"quoted\"'), ('{}')",
        rnd::text()
    ));

    case.assert_same(
        "SELECT name, length(name) FROM users WHERE name = 'O''Brien \\ \"quoted\"'",
        || dsl::quotes_and_backslashes(TRICKY),
    );
}

#[test]
fn non_ascii() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{UNICODE}'), ('{}')",
        rnd::text()
    ));

    case.assert_same(
        &format!("SELECT name, length(name), upper(name) FROM users WHERE name = '{UNICODE}'"),
        || dsl::non_ascii(UNICODE),
    );
}

#[test]
fn typed_literals() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_date, v_timestamp) VALUES ('{}', '{}')",
        literal::date(),
        literal::timestamp(),
    ));

    case.assert_same(
        "SELECT v_date + INTERVAL '1 day 2 hours',
                v_date > DATE '2000-01-01',
                v_timestamp - TIMESTAMP '2000-01-01 00:00:00',
                TIME '10:30',
                NUMERIC '12.50'
         FROM type_samples",
        dsl::typed_literals,
    );
}

#[test]
fn escape_and_dollar_quoted_strings() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('O''Brien \\ \"quoted\"'), ('{}')",
        rnd::text()
    ));

    case.assert_same(
        "SELECT name FROM users
         WHERE name = $$O'Brien \\ \"quoted\"$$
            OR name = E'O\\'Brien \\\\ \"quoted\"'
            OR name = $tag$nested $$ inside$tag$",
        dsl::escape_and_dollar_quoted_strings,
    );
}

#[test]
fn bit_and_hex_literals() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_bit) VALUES ('{}')",
        literal::bit()
    ));

    case.assert_same(
        "SELECT v_bit & B'11110000', v_bit = X'0F', X'FF'::int4, length(B'1010') FROM type_samples",
        dsl::bit_and_hex_literals,
    );
}
