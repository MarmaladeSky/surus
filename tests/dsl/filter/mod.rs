mod dsl;

use crate::support::{Case, rnd};

#[test]
fn by_name() {
    let mut case = Case::new();
    let wanted = rnd::text();
    let other = rnd::text();

    case.exec(&format!(
        "INSERT INTO users (name) VALUES ('{wanted}'), ('{other}')"
    ));

    case.assert_same(
        &format!("SELECT id, name FROM users WHERE name = '{wanted}'"),
        || dsl::by_name(&wanted),
    );
}
