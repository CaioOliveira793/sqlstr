pub struct Columns(pub(crate) &'static str);

pub struct Tables(pub(crate) &'static str);

#[allow(unused_macros)]
macro_rules! comma_separated {
    ($first:literal) => {
        $first
    };

    ($first:literal, $($column:literal),* $(,)?) => {
        concat!($first, $(", ", $column),*)
    };
}

#[allow(unused_imports)]
pub(super) use comma_separated;

#[macro_export]
macro_rules! static_columns {
    ($first:literal, $($column:literal),* $(,)?) => {
        $crate::select::comma_separated!($first, $($column),*)
    };
}

#[macro_export]
macro_rules! static_tables {
    ($first:literal, $($column:literal),* $(,)?) => {
        $crate::select::comma_separated!($first, $($column),*)
    };
}

#[macro_export]
macro_rules! columns {
    ($first:literal, $($column:literal),* $(,)?) => {
        $crate::select::Columns($crate::select::comma_separated!($first, $($column),*))
    };
}

#[macro_export]
macro_rules! tables {
    ($first:literal, $($column:literal),* $(,)?) => {
        $crate::select::Tables($crate::select::comma_separated!($first, $($column),*))
    };
}

// TODO: create high-level sql concat macros to move as much as possible to compile-time

// TODO: create select!() macro to build a sql command in compile-time

pub use columns;
pub use static_columns;
pub use static_tables;
pub use tables;

#[cfg(test)]
mod macro_test {

    #[test]
    fn comma_separated_test() {
        assert_eq!(comma_separated!("id"), "id");
        assert_eq!(comma_separated!("'quoted column'",), "'quoted column'");
        assert_eq!(comma_separated!("id", "name"), "id, name");
        assert_eq!(comma_separated!("id", "name", "age",), "id, name, age");
        assert_eq!(
            comma_separated!("id", "name", "age", "email"),
            "id, name, age, email"
        );
    }
}
