mod dsl;

use crate::support::{Case, rnd};

fn seed(case: &mut Case) -> i32 {
    let room = rnd::int();
    case.exec(&format!(
        "INSERT INTO reservations (room, during) VALUES ({room}, '[2024-01-01,2024-01-10)')"
    ));
    room
}

#[test]
fn non_overlapping_insert() {
    let mut case = Case::new();
    let room = seed(&mut case);

    case.assert_same(
        &format!(
            "INSERT INTO reservations (room, during) VALUES ({room}, '[2024-01-10,2024-01-20)')
             RETURNING room, during"
        ),
        || dsl::non_overlapping_insert(room, "[2024-01-10,2024-01-20)"),
    );
}

#[test]
fn overlap_violation() {
    let mut case = Case::new();
    let room = seed(&mut case);

    case.assert_same_error(
        &format!(
            "INSERT INTO reservations (room, during) VALUES ({room}, '[2024-01-05,2024-01-12)')
             RETURNING room"
        ),
        || dsl::overlap_violation(room, "[2024-01-05,2024-01-12)"),
    );
}
