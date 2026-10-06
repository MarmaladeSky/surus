// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn circle() -> Query {
    t.select(t.v_circle.center()).compile().into()
}

pub fn jsonb_jsonpath() -> Query {
    t.select(t.v_jsonb.matches(t.v_jsonpath)).compile().into()
}

pub fn lseg() -> Query {
    t.select(t.v_lseg.center()).compile().into()
}

pub fn polygon() -> Query {
    t.select(t.v_polygon.center()).compile().into()
}

pub fn r#box() -> Query {
    t.select(t.v_box.center()).compile().into()
}

pub fn text_text() -> Query {
    t.select(t.v_text.matches(t.v_text)).compile().into()
}

pub fn text_tsquery() -> Query {
    t.select(t.v_text.matches(t.v_tsquery)).compile().into()
}

pub fn tsquery_tsvector() -> Query {
    t.select(t.v_tsquery.matches(t.v_tsvector)).compile().into()
}

pub fn tsvector_tsquery() -> Query {
    t.select(t.v_tsvector.matches(t.v_tsquery)).compile().into()
}
