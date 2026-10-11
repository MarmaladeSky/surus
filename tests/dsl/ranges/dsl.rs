use crate::support::placeholder;
use surus::Query;
use surus::types::{Bool, Int4, Multirange, Nullable, Range, Timestamp};

pub fn constructors_and_accessors() -> Query<
    (),
    (
        Range<Int4>,
        Nullable<Range<Int4>>,
        Nullable<Int4>,
        Nullable<Int4>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Timestamp>,
    ),
> {
    placeholder()
}

pub fn containment_and_overlap() -> Query<
    (),
    (
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Bool>,
        Nullable<Range<Int4>>,
        Nullable<Range<Int4>>,
    ),
> {
    placeholder()
}

pub fn multirange_functions() -> Query<
    (),
    (
        Nullable<Int4>,
        Nullable<Int4>,
        Nullable<Bool>,
        Nullable<Multirange<Int4>>,
        Nullable<Range<Int4>>,
    ),
> {
    placeholder()
}

pub fn range_aggregates() -> Query<
    (),
    (
        Nullable<Multirange<Int4>>,
        Nullable<Range<Int4>>,
        Nullable<Multirange<Int4>>,
    ),
> {
    placeholder()
}
