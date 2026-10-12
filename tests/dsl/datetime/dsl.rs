use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types::{Nullable, Timestamp, Timestamptz};

pub fn at_time_zone(
    utc: &str,
    berlin: &str,
) -> Query<(), (Nullable<Timestamp>, Nullable<Timestamptz>)> {
    type_samples
        .select(|s| {
            (
                s.v_timestamptz.at_time_zone(utc),
                s.v_timestamp.at_time_zone(berlin),
            )
        })
        .into()
}
