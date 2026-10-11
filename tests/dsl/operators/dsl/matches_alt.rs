use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn tsquery_tsvector() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn tsvector_tsquery() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
