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

// TODO: support ORDER BY x DESC, y DESC NULL FIRST, z ASC, a USING > NULL LAST
#[allow(unused_macros)]
macro_rules! order_by {
    ($col_expr:literal ASC) => {
        concat!("ORDER BY ", $col_expr, " ASC")
    };
    ($col_expr:literal ASC NULLS FIRST) => {
        concat!("ORDER BY ", $col_expr, " ASC NULLS FIRST")
    };
    ($col_expr:literal ASC NULLS LAST) => {
        concat!("ORDER BY ", $col_expr, " ASC NULLS LAST")
    };

    ($col_expr:literal DESC) => {
        concat!("ORDER BY ", $col_expr, " DESC")
    };
    ($col_expr:literal DESC NULLS FIRST) => {
        concat!("ORDER BY ", $col_expr, " DESC NULLS FIRST")
    };
    ($col_expr:literal DESC NULLS LAST) => {
        concat!("ORDER BY ", $col_expr, " DESC NULLS LAST")
    };

    ($col_expr:literal USING $op:tt) => {
        concat!(
            "ORDER BY ",
            $col_expr,
            " USING ",
            $crate::select::comparison!($op)
        )
    };
}
