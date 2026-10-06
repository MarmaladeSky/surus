// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn tsquery_tsvector() -> Query {
    t.select(t.v_tsquery.matches_deprecated(t.v_tsvector))
        .compile()
        .into()
}

pub fn tsvector_tsquery() -> Query {
    t.select(t.v_tsvector.matches_deprecated(t.v_tsquery))
        .compile()
        .into()
}
