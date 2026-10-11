use crate::support::placeholder;
use surus::Query;
use surus::types::{Array, Bool, Float4, Float8, Int4, Int8, Jsonb, Jsonpath, Nullable, Text};

pub fn typed_parameter_list() -> Query<(Text, Int4, Int8, Bool), (Text, Int4, Int8, Bool)> {
    placeholder()
}

pub fn many_parameters_in()
-> Query<(Text, Text, Text, Text, Text, Text, Text, Text, Text, Text), (Text,)> {
    placeholder()
}

pub fn many_parameters_any() -> Query<(Array<Text>,), (Text,)> {
    placeholder()
}

pub fn parameter_reuse() -> Query<(Text,), (Text, Nullable<Text>)> {
    placeholder()
}

pub fn limit_offset_bound() -> Query<(Int8, Int8), (Text,)> {
    placeholder()
}

pub fn fetch_first_bound() -> Query<(Int8,), (Text,)> {
    placeholder()
}

pub fn generate_series_bound() -> Query<(Int4, Int4), (Text, Int4)> {
    placeholder()
}

pub fn tablesample_bound() -> Query<(Float4, Float8), (Text,)> {
    placeholder()
}

pub fn jsonpath_bound() -> Query<(Jsonpath,), (Nullable<Jsonb>,)> {
    placeholder()
}

pub fn like_pattern_bound() -> Query<(Text,), (Text,)> {
    placeholder()
}

pub fn any_array_bound() -> Query<(Array<Int8>,), (Text,)> {
    placeholder()
}

pub fn parameter_reuse_single_slot() -> Query<(Text,), (Int8,)> {
    placeholder()
}

pub fn hostile_text() -> Query<(Text,), (Text,)> {
    placeholder()
}
