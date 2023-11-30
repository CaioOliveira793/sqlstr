// TODO: support all the postgres dialect
// https://www.postgresql.org/docs/9.0/functions.html

#[macro_export]
macro_rules! group_by {
    ($first:literal$(,)? $($column:literal),* $(,)?) => {
        concat!("GROUP BY ", $first, $(", ", $column),*)
    };
    (ALL $first:literal$(,)? $($column:literal),* $(,)?) => {
        concat!("GROUP BY ALL ", $first, $(", ", $column),*)
    };
    (DISTINCT $first:literal$(,)? $($column:literal),* $(,)?) => {
        concat!("GROUP BY DISTINCT ", $first, $(", ", $column),*)
    };
}

#[macro_export]
macro_rules! grouping_element {
    ($first:literal$(,)? $($column:literal),* $(,)?) => {
        concat!($first, $(", ", $column),*)
    };
    (ROLLUP ($first:literal$(,)? $($column:literal),*)) => {
        concat!("ROLLUP (", $first, $(", ", $column),*, ")")
    };
    (CUBE ($first:literal$(,)? $($column:literal),*)) => {
        concat!("CUBE (", $first, $(", ", $column),*, ")")
    };
    (GROUPING SETS ($first:literal$(,)? $($column:literal),*)) => {
        concat!("GROUPING SETS (", $first, $(", ", $column),*, ")")
    };
}
