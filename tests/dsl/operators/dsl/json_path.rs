use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn json_text_array() -> Query<(), (t::Nullable<t::Json>,)> {
    placeholder()
}

pub fn jsonb_text_array() -> Query<(), (t::Nullable<t::Jsonb>,)> {
    placeholder()
}
