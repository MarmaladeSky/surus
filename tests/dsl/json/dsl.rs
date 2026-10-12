use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Bytea, Int4, Json, Jsonb, Nullable, Text};

pub fn jsonb_build_object() -> Query<(), (Jsonb,)> {
    placeholder()
}

pub fn jsonb_set() -> Query<(), (Nullable<Jsonb>, Nullable<Jsonb>)> {
    placeholder()
}

pub fn sql_json_constructors() -> Query<(), (Json, Json)> {
    placeholder()
}

pub fn json_table() -> Query<(), (Nullable<Int4>, Nullable<Text>)> {
    placeholder()
}

pub fn is_json_predicates() -> Query<
    (),
    (
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
    ),
> {
    placeholder()
}

pub fn json_exists_query_value() -> Query<
    (),
    (
        Nullable<Bool>,
        Nullable<Jsonb>,
        Nullable<Int4>,
        Nullable<Text>,
        Nullable<Text>,
        Nullable<Jsonb>,
    ),
> {
    placeholder()
}

pub fn json_serialize_and_constructors() -> Query<
    (),
    (
        Nullable<Text>,
        Nullable<Bytea>,
        Nullable<Json>,
        Nullable<Json>,
        Nullable<Json>,
    ),
> {
    placeholder()
}

pub fn jsonb_aggregates() -> Query<(), (Nullable<Jsonb>, Nullable<Jsonb>, Nullable<Json>)> {
    placeholder()
}

pub fn jsonb_path_functions() -> Query<(), (Nullable<Jsonb>, Nullable<Jsonb>, Nullable<Bool>)> {
    placeholder()
}

pub fn whole_row_aggregate() -> Query<(), (Nullable<Jsonb>,)> {
    placeholder()
}
