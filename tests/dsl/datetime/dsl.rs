use crate::support::Query;
use schema::*;
use surus::*;

pub fn at_time_zone(utc: &str, berlin: &str) -> Query {
    let t = type_samples;
    t.select((
        t.v_timestamptz.at_time_zone(utc),
        t.v_timestamp.at_time_zone(berlin),
    ))
    .compile()
    .into()
}
