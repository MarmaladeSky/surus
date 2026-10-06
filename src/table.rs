//! Macros used by generated schema definitions.

use std::marker::PhantomData;

/// A named constraint of table `Q`, for `ON CONFLICT ON CONSTRAINT`.
pub struct Constraint<Q> {
    pub(crate) name: &'static str,
    _table: PhantomData<fn() -> Q>,
}

impl<Q> Constraint<Q> {
    pub const fn new(name: &'static str) -> Self {
        Constraint {
            name,
            _table: PhantomData,
        }
    }
}

impl<Q> Clone for Constraint<Q> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Q> Copy for Constraint<Q> {}

impl crate::query::FromItem {
    /// A table, aliased when `qualifier` differs from its name.
    #[doc(hidden)]
    pub fn table(schema: &'static str, name: &'static str, qualifier: &'static str) -> Self {
        let mut item = Self::new(crate::query::FromKind::Table(schema, name));
        if qualifier != name {
            item.alias = Some(qualifier);
        }
        item
    }
}

/// Declares a table or view: `value: Struct(Marker) = "schema"."name" { field: "column" Type, Nullability; ... }`.
#[macro_export]
macro_rules! table {
    ($value:ident: $Struct:ident($Marker:ident) = $schema:literal . $name:literal {
        $($field:ident: $column:literal $t:ty, $n:ty;)*
    }) => {
        #[doc = concat!("Table `", $schema, ".", $name, "`.")]
        pub struct $Struct<Q = $Marker> {
            $(#[doc = concat!("Column `", $column, "`.")] pub $field: $crate::Column<$t, $n, Q>,)*
            qualifier: &'static str,
        }

        #[doc = concat!("Identity of table `", $schema, ".", $name, "`.")]
        pub enum $Marker {}

        impl $crate::Qualifier for $Marker {
            type Null = $crate::NotNull;
        }

        #[allow(non_upper_case_globals)]
        #[doc = concat!("Table `", $schema, ".", $name, "`.")]
        pub const $value: $Struct = $Struct::with_qualifier($name);

        impl<Q> $Struct<Q> {
            const fn with_qualifier(qualifier: &'static str) -> Self {
                $Struct { $($field: $crate::Column::new(qualifier, $column),)* qualifier }
            }
        }

        impl<Q> Clone for $Struct<Q> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<Q> Copy for $Struct<Q> {}

        impl<Q: $crate::Qualifier> $crate::Source for $Struct<Q> {
            type Id = Q;
            type Scope = ();
            type Row = ($(<<Q::Null as $crate::Nullability>::Or<$n> as $crate::Nullability>::Field<$t>,)*);

            fn into_from_item(self) -> $crate::FromItem {
                $crate::FromItem::table($schema, $name, self.qualifier)
            }

            fn qualifier(&self) -> &'static str {
                self.qualifier
            }
        }

        impl<Q: $crate::Qualifier> $crate::Requalify for $Struct<Q> {
            type As<Q2: $crate::Qualifier> = $Struct<Q2>;

            fn requalify<Q2: $crate::Qualifier>(self, name: Option<&'static str>) -> $Struct<Q2> {
                $Struct::with_qualifier(name.unwrap_or(self.qualifier))
            }
        }

        impl<Q: $crate::Qualifier> $crate::Table for $Struct<Q> {}
    };
}

/// Declares an enum type.
#[macro_export]
macro_rules! sql_enum {
    ($name:ident = $sql:literal) => {
        #[doc = concat!("Enum type `", $sql, "`.")]
        pub struct $name;
        $crate::sql_type!($name = $sql);
        impl $crate::Enum for $name {}
    };
}

/// Declares a domain over `base`; values coerce to and from the base type.
#[macro_export]
macro_rules! domain {
    ($name:ident = $sql:literal: $base:ty) => {
        #[doc = concat!("Domain `", $sql, "`.")]
        pub struct $name;
        $crate::sql_type!($name = $sql);
        impl $crate::Coerce<$base> for $name {}
        impl $crate::Coerce<$name> for $base {}
    };
}

/// Declares a composite type with field accessors in trait `$Fields`.
#[macro_export]
macro_rules! composite {
    ($name:ident = $sql:literal, $Fields:ident { $($field:ident: $column:literal $t:ty;)* }) => {
        #[doc = concat!("Composite type `", $sql, "`.")]
        pub struct $name;
        $crate::sql_type!($name = $sql);

        impl $crate::Composite for $name {
            type Row = ($(Option<$t>,)*);
        }

        #[allow(non_camel_case_types)]
        impl<$($field: $crate::Field),*> $crate::CastTo<$name> for $crate::Record<($($field,)*)>
        where
            $($field::Sql: $crate::CastTo<$t>,)*
        {}

        #[doc = concat!("Fields of composite type `", $sql, "`.")]
        pub trait $Fields: $crate::Expression<Sql = $name> {
            $(
                #[doc = concat!("Field `", $column, "`.")]
                fn $field(self) -> $crate::Expr<$t, $crate::Nullable, Self::Scope> {
                    $crate::field(self, $column)
                }
            )*
        }

        impl<E: $crate::Expression<Sql = $name>> $Fields for E {}
    };
}

/// Declares binary operators: `Op: Left, Right => Result;`.
#[doc(hidden)]
#[macro_export]
macro_rules! binary_operators {
    ($($op:ident: $l:ty, $r:ty => $out:ty;)*) => {$(
        impl $crate::Operator<$crate::op::$op, $r> for $l {
            type Out = $out;
        }
    )*};
}

/// Declares prefix operators: `Op "symbol": Operand => Result;`.
#[doc(hidden)]
#[macro_export]
macro_rules! prefix_operators {
    ($($op:ident $sql:literal: $t:ty => $out:ty;)*) => {$(
        impl $crate::Prefix<$crate::op::$op> for $t {
            type Out = $out;
            const SQL: &'static str = $sql;
        }
    )*};
}

/// Declares casts: `From => To, ...;`.
#[doc(hidden)]
#[macro_export]
macro_rules! casts {
    ($($from:ty => $($to:ty),+;)*) => {$($(
        impl $crate::CastTo<$to> for $from {}
    )+)*};
}

/// Declares implicit coercions: `From => To, ...;`.
#[doc(hidden)]
#[macro_export]
macro_rules! coercions {
    ($($from:ty => $($to:ty),+;)*) => {$($(
        impl $crate::Coerce<$to> for $from {}
    )+)*};
}
