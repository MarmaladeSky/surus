mod dsl;

use crate::support::{Case, compile, rnd, sql};
use surus::Query;
use surus::types::{Int4, Range, Timestamp};

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

    let expected: Query<(Int4,), (Int4, Range<Timestamp>)> = sql(
        "INSERT INTO reservations (room, during) VALUES ($1, '[2024-01-10,2024-01-20)')
             RETURNING room, during",
    );
    case.assert_same(
        expected,
        compile(|| dsl::non_overlapping_insert("[2024-01-10,2024-01-20)")),
        (room,),
    );
}

#[test]
fn overlap_violation() {
    let mut case = Case::new();
    let room = seed(&mut case);

    let expected: Query<(Int4,), (Int4,)> = sql(
        "INSERT INTO reservations (room, during) VALUES ($1, '[2024-01-05,2024-01-12)')
             RETURNING room",
    );
    case.assert_same_error(
        expected,
        compile(|| dsl::overlap_violation("[2024-01-05,2024-01-12)")),
        (room,),
    );
}
