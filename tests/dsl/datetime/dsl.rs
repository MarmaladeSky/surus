use crate::support::placeholder;
use surus::Query;
use surus::types::{Int4, Nullable, Timestamp, Timestamptz};

pub fn at_time_zone(
    _utc: &str,
    _berlin: &str,
) -> Query<(), (Nullable<Timestamp>, Nullable<Timestamptz>)> {
    placeholder()
}

pub fn before_now() -> Query<(), (Nullable<Int4>,)> {
    placeholder()
}
