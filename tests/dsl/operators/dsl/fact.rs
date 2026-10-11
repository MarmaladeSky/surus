use crate::support::placeholder;
use surus::Query;
use surus::types as t;

pub fn tsquery() -> Query<(), (t::Nullable<t::Tsquery>,)> {
    placeholder()
}
