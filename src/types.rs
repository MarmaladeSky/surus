mod catalog;

pub use catalog::*;
use std::marker::PhantomData;

pub trait PgType {
    fn name() -> String;
}

pub trait RangeSubtype: PgType {
    const RANGE: &'static str;
    const MULTIRANGE: &'static str;
}

pub trait Element: PgType {}

pub struct Array<T>(PhantomData<T>);
pub struct Range<T>(PhantomData<T>);
pub struct Multirange<T>(PhantomData<T>);
pub struct Nullable<T>(PhantomData<T>);

impl<T: Element> PgType for Array<T> {
    fn name() -> String {
        format!("{}[]", T::name())
    }
}

impl<T: RangeSubtype> PgType for Range<T> {
    fn name() -> String {
        T::RANGE.to_string()
    }
}

impl<T: RangeSubtype> Element for Range<T> {}

impl<T: RangeSubtype> PgType for Multirange<T> {
    fn name() -> String {
        T::MULTIRANGE.to_string()
    }
}

impl<T: RangeSubtype> Element for Multirange<T> {}

impl<T: PgType> PgType for Nullable<T> {
    fn name() -> String {
        T::name()
    }
}

impl<T: Element> Element for Nullable<T> {}

pub struct Record;

impl PgType for Record {
    fn name() -> String {
        "record".to_string()
    }
}

impl Element for Record {}
