mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Int8, Text};

#[test]
fn by_name() {
    let mut case = Case::new();
    let wanted = rnd::text();
    let other = rnd::text();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{wanted}'), ('{other}')"
    ));

    let expected: Query<(Text,), (Int8, Text)> = sql("SELECT id, name FROM users WHERE name = $1");
    case.assert_same(expected, compile(dsl::by_name), (&wanted,));
}
