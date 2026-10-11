use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn price_tag_price_tag() -> Query<(), (t::Nullable<t::Bool>,)> {
    placeholder()
}
