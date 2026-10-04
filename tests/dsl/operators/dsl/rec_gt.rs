use crate::support::Query;

pub fn price_tag_price_tag() -> Query {
    Query::plain("SELECT NULL WHERE false")
}
