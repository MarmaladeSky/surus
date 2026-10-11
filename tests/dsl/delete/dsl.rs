use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Int8, Nullable, Text};

pub fn by_name() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn delete_using() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn returning_table_star() -> Query<(Text,), (Int8, Text, Nullable<Text>, Nullable<Int8>, Text)>
{
    placeholder()
}

pub fn returning_old() -> Query<(Text,), (Text, Nullable<Text>, Bool)> {
    placeholder()
}

pub fn delete_using_subquery() -> Query<(), (Text,)> {
    placeholder()
}
