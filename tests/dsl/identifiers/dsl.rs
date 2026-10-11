use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Int4, Nullable, Numeric, Text};

pub fn quoted_table_and_columns() -> Query<(Text, Int4), (Bool, Text, Int4, Nullable<Numeric>)> {
    placeholder()
}

pub fn quoted_aliases() -> Query<(), (Text, Int4)> {
    placeholder()
}

pub fn schema_qualified_tables() -> Query<(), (Text, Text)> {
    placeholder()
}
