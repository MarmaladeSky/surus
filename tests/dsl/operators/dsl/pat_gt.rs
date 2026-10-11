use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn bpchar_bpchar() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}

pub fn text_text() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
