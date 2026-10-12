mod dsl;

use crate::support::{Case, compile, literal, rnd, sql};
use surus::Query;
use surus::types::{Int4, Nullable, Timestamp, Timestamptz};

#[test]
fn at_time_zone() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_timestamp, v_timestamptz) VALUES ('{}', '{}')",
        literal::timestamp(),
        literal::timestamptz(),
    ));

    let expected: Query<(), (Nullable<Timestamp>, Nullable<Timestamptz>)> = sql(
        "SELECT v_timestamptz AT TIME ZONE 'UTC', v_timestamp AT TIME ZONE 'Europe/Berlin' FROM type_samples",
    );
    case.assert_same(
        expected,
        compile(|| dsl::at_time_zone("UTC", "Europe/Berlin")),
        (),
    );
}

#[test]
fn before_now() {
    let mut case = Case::new();
    let past = rnd::int();
    case.exec(&format!(
        "INSERT INTO type_samples (v_int4, v_timestamptz)
         VALUES ({past}, '2000-01-01 00:00:00+00'), ({}, '2999-01-01 00:00:00+00')",
        past + 1,
    ));

    let expected: Query<(), (Nullable<Int4>,)> =
        sql("SELECT v_int4 FROM type_samples WHERE v_timestamptz < now() ORDER BY v_int4");
    case.assert_same(expected, compile(dsl::before_now), ());
}
