mod dsl;

use crate::support::{Case, literal};

#[test]
fn at_time_zone() {
    let mut case = Case::new();
    case.exec(&format!(
        "INSERT INTO type_samples (v_timestamp, v_timestamptz) VALUES ('{}', '{}')",
        literal::timestamp(),
        literal::timestamptz(),
    ));

    case.assert_same(
        "SELECT v_timestamptz AT TIME ZONE 'UTC', v_timestamp AT TIME ZONE 'Europe/Berlin' FROM type_samples",
        || dsl::at_time_zone("UTC", "Europe/Berlin"),
    );
}
