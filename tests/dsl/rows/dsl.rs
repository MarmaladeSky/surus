use crate::schema::PriceTag;
use crate::support::placeholder;
use surus::Query;
use surus::types::{Nullable, Numeric, Record, Text};

pub fn constructor() -> Query<(), (Record, Record)> {
    placeholder()
}

pub fn comparison(_price_cents: i32, _quantity: i32) -> Query<(), (Text,)> {
    placeholder()
}

pub fn composite_field_access() -> Query<(), (Nullable<Numeric>, Nullable<Text>, Nullable<PriceTag>)>
{
    placeholder()
}

pub fn composite_constructor_cast() -> Query<(), (PriceTag, Nullable<Text>)> {
    placeholder()
}

pub fn composite_expansion() -> Query<(), (Nullable<Numeric>, Nullable<Text>)> {
    placeholder()
}
