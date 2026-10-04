mod dsl;

use crate::support::{Case, rnd};

#[test]
fn check_violation() {
    let mut case = Case::new();
    let name = rnd::text();

    case.assert_same_error(
        &format!("INSERT INTO products (name, price_cents) VALUES ('{name}', -1) RETURNING name"),
        || dsl::check_violation(&name, -1),
    );
}

#[test]
fn unique_violation() {
    let mut case = Case::new();
    let batch = rnd::text();
    let name = rnd::text();
    case.exec(&format!(
        "INSERT INTO tickets (batch, name) VALUES ('{batch}', '{name}')"
    ));

    case.assert_same_error(
        &format!("INSERT INTO tickets (batch, name) VALUES ('{batch}', '{name}') RETURNING id"),
        || dsl::unique_violation(&batch, &name),
    );
}

#[test]
fn on_conflict_on_constraint() {
    let mut case = Case::new();
    let batch = rnd::text();
    let name = rnd::text();
    case.exec(&format!(
        "INSERT INTO tickets (batch, name) VALUES ('{batch}', '{name}')"
    ));

    case.assert_same(
        &format!(
            "INSERT INTO tickets (batch, name) VALUES ('{batch}', '{name}')
             ON CONFLICT ON CONSTRAINT tickets_batch_name_key DO UPDATE SET name = tickets.name || '!'
             RETURNING batch, name"
        ),
        || dsl::on_conflict_on_constraint(&batch, &name),
    );
}
