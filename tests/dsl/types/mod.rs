mod dsl;

use crate::support::{Case, Param, Query, TextParam, literal};
use std::panic::{AssertUnwindSafe, catch_unwind};

struct TypeCase {
    column: &'static str,
    value: fn() -> String,
    dsl: fn(Param) -> Query,
}

const TYPES: &[TypeCase] = &[
    TypeCase {
        column: "v_bool",
        value: literal::bool,
        dsl: dsl::bool,
    },
    TypeCase {
        column: "v_int2",
        value: literal::int2,
        dsl: dsl::int2,
    },
    TypeCase {
        column: "v_int4",
        value: literal::int4,
        dsl: dsl::int4,
    },
    TypeCase {
        column: "v_int8",
        value: literal::int8,
        dsl: dsl::int8,
    },
    TypeCase {
        column: "v_numeric",
        value: literal::numeric,
        dsl: dsl::numeric,
    },
    TypeCase {
        column: "v_float4",
        value: literal::float4,
        dsl: dsl::float4,
    },
    TypeCase {
        column: "v_float8",
        value: literal::float8,
        dsl: dsl::float8,
    },
    TypeCase {
        column: "v_money",
        value: literal::money,
        dsl: dsl::money,
    },
    TypeCase {
        column: "v_text",
        value: literal::text,
        dsl: dsl::text,
    },
    TypeCase {
        column: "v_varchar",
        value: literal::varchar,
        dsl: dsl::varchar,
    },
    TypeCase {
        column: "v_bpchar",
        value: literal::bpchar,
        dsl: dsl::bpchar,
    },
    TypeCase {
        column: "v_char",
        value: literal::char,
        dsl: dsl::char,
    },
    TypeCase {
        column: "v_date",
        value: literal::date,
        dsl: dsl::date,
    },
    TypeCase {
        column: "v_time",
        value: literal::time,
        dsl: dsl::time,
    },
    TypeCase {
        column: "v_timetz",
        value: literal::timetz,
        dsl: dsl::timetz,
    },
    TypeCase {
        column: "v_timestamp",
        value: literal::timestamp,
        dsl: dsl::timestamp,
    },
    TypeCase {
        column: "v_timestamptz",
        value: literal::timestamptz,
        dsl: dsl::timestamptz,
    },
    TypeCase {
        column: "v_interval",
        value: literal::interval,
        dsl: dsl::interval,
    },
    TypeCase {
        column: "v_bytea",
        value: literal::bytea,
        dsl: dsl::bytea,
    },
    TypeCase {
        column: "v_uuid",
        value: literal::uuid,
        dsl: dsl::uuid,
    },
    TypeCase {
        column: "v_json",
        value: literal::json,
        dsl: dsl::json,
    },
    TypeCase {
        column: "v_jsonb",
        value: literal::jsonb,
        dsl: dsl::jsonb,
    },
    TypeCase {
        column: "v_jsonpath",
        value: literal::jsonpath,
        dsl: dsl::jsonpath,
    },
    TypeCase {
        column: "v_inet",
        value: literal::inet,
        dsl: dsl::inet,
    },
    TypeCase {
        column: "v_cidr",
        value: literal::cidr,
        dsl: dsl::cidr,
    },
    TypeCase {
        column: "v_macaddr",
        value: literal::macaddr,
        dsl: dsl::macaddr,
    },
    TypeCase {
        column: "v_macaddr8",
        value: literal::macaddr8,
        dsl: dsl::macaddr8,
    },
    TypeCase {
        column: "v_bit",
        value: literal::bit,
        dsl: dsl::bit,
    },
    TypeCase {
        column: "v_varbit",
        value: literal::varbit,
        dsl: dsl::varbit,
    },
    TypeCase {
        column: "v_point",
        value: literal::point,
        dsl: dsl::point,
    },
    TypeCase {
        column: "v_line",
        value: literal::line,
        dsl: dsl::line,
    },
    TypeCase {
        column: "v_lseg",
        value: literal::lseg,
        dsl: dsl::lseg,
    },
    TypeCase {
        column: "v_box",
        value: literal::r#box,
        dsl: dsl::r#box,
    },
    TypeCase {
        column: "v_circle",
        value: literal::circle,
        dsl: dsl::circle,
    },
    TypeCase {
        column: "v_path",
        value: literal::path,
        dsl: dsl::path,
    },
    TypeCase {
        column: "v_polygon",
        value: literal::polygon,
        dsl: dsl::polygon,
    },
    TypeCase {
        column: "v_tsvector",
        value: literal::tsvector,
        dsl: dsl::tsvector,
    },
    TypeCase {
        column: "v_tsquery",
        value: literal::tsquery,
        dsl: dsl::tsquery,
    },
    TypeCase {
        column: "v_xml",
        value: literal::xml,
        dsl: dsl::xml,
    },
    TypeCase {
        column: "v_int4_array",
        value: literal::int4_array,
        dsl: dsl::int4_array,
    },
    TypeCase {
        column: "v_text_array",
        value: literal::text_array,
        dsl: dsl::text_array,
    },
    TypeCase {
        column: "v_int4range",
        value: literal::int4range,
        dsl: dsl::int4range,
    },
    TypeCase {
        column: "v_tsrange",
        value: literal::tsrange,
        dsl: dsl::tsrange,
    },
    TypeCase {
        column: "v_int4multirange",
        value: literal::int4multirange,
        dsl: dsl::int4multirange,
    },
    TypeCase {
        column: "v_mood",
        value: literal::mood,
        dsl: dsl::mood,
    },
    TypeCase {
        column: "v_positive_int",
        value: literal::positive_int,
        dsl: dsl::positive_int,
    },
    TypeCase {
        column: "v_price_tag",
        value: literal::price_tag,
        dsl: dsl::price_tag,
    },
];

#[test]
fn round_trip() {
    let mut case = Case::new();
    let failed: Vec<&str> = TYPES
        .iter()
        .filter(|t| {
            let value = (t.value)();
            let plain = format!(
                "INSERT INTO type_samples ({c}) VALUES ('{value}') RETURNING {c}",
                c = t.column
            );
            catch_unwind(AssertUnwindSafe(|| {
                case.assert_same(&plain, || (t.dsl)(Box::new(TextParam(value))))
            }))
            .is_err()
        })
        .map(|t| t.column)
        .collect();
    assert!(failed.is_empty(), "failed types: {failed:?}");
}
