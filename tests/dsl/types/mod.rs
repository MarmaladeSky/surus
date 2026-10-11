mod dsl;

use crate::support::{Case, Input, compile, literal, sql};
use std::panic::{AssertUnwindSafe, catch_unwind};
use surus::Query;
use surus::types::{Nullable, PgType};

fn check<T: PgType>(
    column: &'static str,
    value: fn() -> String,
    dsl: impl FnOnce() -> Query<(T,), (Nullable<T>,)>,
) -> Option<&'static str> {
    let mut case = Case::new();
    let value = Input(value());
    let insert = format!("INSERT INTO type_samples ({column}) VALUES ($1) RETURNING {column}");
    catch_unwind(AssertUnwindSafe(|| {
        case.assert_same(sql(&insert), compile(dsl), (value,))
    }))
    .is_err()
    .then_some(column)
}

#[test]
fn round_trip() {
    let failed: Vec<&str> = [
        check("v_bool", literal::bool, dsl::bool),
        check("v_int2", literal::int2, dsl::int2),
        check("v_int4", literal::int4, dsl::int4),
        check("v_int8", literal::int8, dsl::int8),
        check("v_numeric", literal::numeric, dsl::numeric),
        check("v_float4", literal::float4, dsl::float4),
        check("v_float8", literal::float8, dsl::float8),
        check("v_money", literal::money, dsl::money),
        check("v_text", literal::text, dsl::text),
        check("v_varchar", literal::varchar, dsl::varchar),
        check("v_bpchar", literal::bpchar, dsl::bpchar),
        check("v_char", literal::char, dsl::char),
        check("v_date", literal::date, dsl::date),
        check("v_time", literal::time, dsl::time),
        check("v_timetz", literal::timetz, dsl::timetz),
        check("v_timestamp", literal::timestamp, dsl::timestamp),
        check("v_timestamptz", literal::timestamptz, dsl::timestamptz),
        check("v_interval", literal::interval, dsl::interval),
        check("v_bytea", literal::bytea, dsl::bytea),
        check("v_uuid", literal::uuid, dsl::uuid),
        check("v_json", literal::json, dsl::json),
        check("v_jsonb", literal::jsonb, dsl::jsonb),
        check("v_jsonpath", literal::jsonpath, dsl::jsonpath),
        check("v_inet", literal::inet, dsl::inet),
        check("v_cidr", literal::cidr, dsl::cidr),
        check("v_macaddr", literal::macaddr, dsl::macaddr),
        check("v_macaddr8", literal::macaddr8, dsl::macaddr8),
        check("v_bit", literal::bit, dsl::bit),
        check("v_varbit", literal::varbit, dsl::varbit),
        check("v_point", literal::point, dsl::point),
        check("v_line", literal::line, dsl::line),
        check("v_lseg", literal::lseg, dsl::lseg),
        check("v_box", literal::r#box, dsl::r#box),
        check("v_circle", literal::circle, dsl::circle),
        check("v_path", literal::path, dsl::path),
        check("v_polygon", literal::polygon, dsl::polygon),
        check("v_tsvector", literal::tsvector, dsl::tsvector),
        check("v_tsquery", literal::tsquery, dsl::tsquery),
        check("v_xml", literal::xml, dsl::xml),
        check("v_int4_array", literal::int4_array, dsl::int4_array),
        check("v_text_array", literal::text_array, dsl::text_array),
        check("v_int4range", literal::int4range, dsl::int4range),
        check("v_tsrange", literal::tsrange, dsl::tsrange),
        check(
            "v_int4multirange",
            literal::int4multirange,
            dsl::int4multirange,
        ),
        check("v_mood", literal::mood, dsl::mood),
        check("v_positive_int", literal::positive_int, dsl::positive_int),
        check("v_price_tag", literal::price_tag, dsl::price_tag),
    ]
    .into_iter()
    .flatten()
    .collect();
    assert!(failed.is_empty(), "failed types: {failed:?}");
}
