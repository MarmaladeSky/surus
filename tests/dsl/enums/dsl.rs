use crate::schema::{Mood, type_samples};
use surus::prelude::*;
use surus::types::{Array, Nullable};
use surus::{
    Query, enum_first, enum_last, enum_range, enum_range_between, literal, null, typed_null,
};

pub fn declaration_order() -> Query<(), (Nullable<Mood>,)> {
    type_samples
        .select(|s| s.v_mood)
        .order_by(|s| s.v_mood)
        .into()
}

pub fn comparison_with_parameter(mood: &str) -> Query<(), (Nullable<Mood>,)> {
    type_samples
        .filter(|s| s.v_mood.ge(literal::<Mood>(mood)))
        .select(|s| s.v_mood)
        .order_by(|s| s.v_mood)
        .into()
}

pub fn enum_functions() -> Query<
    (),
    (
        Nullable<Mood>,
        Nullable<Array<Mood>>,
        Nullable<Mood>,
        Nullable<Mood>,
        Nullable<Array<Mood>>,
    ),
> {
    type_samples
        .select(|s| {
            (
                s.v_mood,
                enum_range_between(s.v_mood, null()),
                enum_first(s.v_mood),
                enum_last(s.v_mood),
                enum_range(typed_null::<Mood>()),
            )
        })
        .order_by(|s| s.v_mood)
        .into()
}
