#![allow(clippy::eq_op)]

use crate::schema::type_samples;
use surus::Query;
use surus::prelude::*;
use surus::types as t;

pub fn circle() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples.select(|s| s.v_circle.center()).into()
}

pub fn jsonb_jsonpath() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_jsonb.matches(s.v_jsonpath))
        .into()
}

pub fn lseg() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples.select(|s| s.v_lseg.center()).into()
}

pub fn polygon() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples.select(|s| s.v_polygon.center()).into()
}

pub fn r#box() -> Query<(), (t::Nullable<t::Point>,)> {
    type_samples.select(|s| s.v_box.center()).into()
}

pub fn text_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples.select(|s| s.v_text.matches(s.v_text)).into()
}

pub fn text_tsquery() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_text.matches(s.v_tsquery))
        .into()
}

pub fn tsquery_tsvector() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsquery.matches(s.v_tsvector))
        .into()
}

pub fn tsvector_tsquery() -> Query<(), (t::Nullable<t::Bool>,)> {
    type_samples
        .select(|s| s.v_tsvector.matches(s.v_tsquery))
        .into()
}
