use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn circle() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn jsonb_jsonpath() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn lseg() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn polygon() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn r#box() -> Query<(), (t::Nullable<t::Point>,)> {
    placeholder()
}

pub fn text_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn text_tsquery() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn tsquery_tsvector() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn tsvector_tsquery() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
