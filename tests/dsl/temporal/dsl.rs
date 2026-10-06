use crate::support::Query;
use schema::*;
use surus::*;

pub fn non_overlapping_insert(room: i32, during: &str) -> Query {
    reservations
        .insert((reservations.room, reservations.during))
        .values((room, param(during).cast::<Range<Timestamp>>()))
        .returning((reservations.room, reservations.during))
        .compile()
        .into()
}

pub fn overlap_violation(room: i32, during: &str) -> Query {
    reservations
        .insert((reservations.room, reservations.during))
        .values((room, param(during).cast::<Range<Timestamp>>()))
        .returning(reservations.room)
        .compile()
        .into()
}
