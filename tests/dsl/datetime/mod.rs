mod dsl;

use crate::support::{Case, compile, literal, sql};
use surus::Query;
use surus::types::{Nullable, Timestamp, Timestamptz};

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
