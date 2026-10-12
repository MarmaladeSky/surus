use crate::schema::reservations;
use surus::prelude::*;
use surus::types::{Int4, Range, Timestamp};
use surus::{Query, literal, query};

pub fn non_overlapping_insert(during: &str) -> Query<(Int4,), (Int4, Range<Timestamp>)> {
    query(|(room,)| {
        reservations
            .insert(|r| r.room(room).during(literal::<Range<Timestamp>>(during)))
            .returning(|r| (r.room, r.during))
    })
}

pub fn overlap_violation(during: &str) -> Query<(Int4,), (Int4,)> {
    query(|(room,)| {
        reservations
            .insert(|r| r.room(room).during(literal::<Range<Timestamp>>(during)))
            .returning(|r| r.room)
    })
}
