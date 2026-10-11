use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn json_int4() -> Query<(), (t::Nullable<t::Text>,)> {
    placeholder()
}

pub fn json_text() -> Query<(), (t::Nullable<t::Text>,)> {
    placeholder()
}

pub fn jsonb_int4() -> Query<(), (t::Nullable<t::Text>,)> {
    placeholder()
}

pub fn jsonb_text() -> Query<(), (t::Nullable<t::Text>,)> {
    placeholder()
}
