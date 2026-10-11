use crate::support::placeholder;
use surus::Query;
use surus::types::{Nullable, Timestamp, Timestamptz};

pub fn at_time_zone(
    _utc: &str,
    _berlin: &str,
) -> Query<(), (Nullable<Timestamp>, Nullable<Timestamptz>)> {
    placeholder()
}
