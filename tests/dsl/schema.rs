// Stand-in for the generated typed schema of the fixtures in schema/
use surus::types::{Element, PgType};

pub struct Mood;
pub struct PositiveInt;
pub struct PriceTag;

impl PgType for Mood {
    fn name() -> String {
        "mood".to_string()
    }
}

impl Element for Mood {}

impl PgType for PositiveInt {
    fn name() -> String {
        "positive_int".to_string()
    }
}

impl Element for PositiveInt {}

impl PgType for PriceTag {
    fn name() -> String {
        "price_tag".to_string()
    }
}

impl Element for PriceTag {}
