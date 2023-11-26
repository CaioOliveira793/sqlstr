#[macro_export]
macro_rules! sql_expr {
    (expr($expr:expr)) => {
        $crate::command::SqlExpr::Expr($expr) as $crate::command::SqlExpr<'_, ()>
    };
    (expr($expr:expr), $($rest:tt),*) => {
        (
            $crate::command::SqlExpr::Expr($expr) as $crate::command::SqlExpr<'_, ()>,
            $($crate::util::sql_expr!($rest)),*
        )
    };

    (val($val:expr)) => {
        $crate::command::SqlExpr::Value($val)
    };
    (val($val:expr), $($rest:tt),*) => {
        $crate::command::SqlExpr::Value($val),
        $($crate::util::sql_expr!($rest)),*
    };

    ($expr:literal) => {
        $crate::command::SqlExpr::Expr($expr) as $crate::command::SqlExpr<'_, ()>
    };
    ($expr:literal, $($rest:tt),*) => {
        (
            $crate::command::SqlExpr::Expr($expr) as $crate::command::SqlExpr<'_, ()>,
            $($crate::util::sql_expr!($rest)),*
        )
    };
}

#[macro_export]
macro_rules! cast {
    ($expr:literal AS $ty:literal) => {
        concat!("CAST (", $expr, " AS ", $ty, ")")
    };
    (expr($expr:expr) AS $ty:expr) => {
        (
            $crate::command::SqlExpr::Expr("CAST (") as $crate::command::SqlExpr<'_, ()>,
            $crate::command::SqlExpr::Expr($expr) as $crate::command::SqlExpr<'_, ()>,
            $crate::command::SqlExpr::Expr(" AS ") as $crate::command::SqlExpr<'_, ()>,
            $crate::command::SqlExpr::Expr($ty) as $crate::command::SqlExpr<'_, ()>,
            $crate::command::SqlExpr::Expr(")") as $crate::command::SqlExpr<'_, ()>,
        )
    };
    (val($expr:expr) AS $ty:expr) => {
        (
            $crate::command::SqlExpr::Expr("CAST (") as $crate::command::SqlExpr<'_, ()>,
            $crate::command::SqlExpr::Value($expr),
            $crate::command::SqlExpr::Expr(" AS ") as $crate::command::SqlExpr<'_, ()>,
            $crate::command::SqlExpr::Expr($ty) as $crate::command::SqlExpr<'_, ()>,
            $crate::command::SqlExpr::Expr(")") as $crate::command::SqlExpr<'_, ()>,
        )
    };
}

pub use cast;

#[cfg(test)]
mod test {
    use super::*;
    use crate::command::SqlExpr;
    use SqlExpr::*;

    #[test]
    fn sql_expr_test() {
        assert_eq!(sql_expr!(""), (Expr("")));
        assert_eq!(sql_expr!(expr("IN")), (Expr("IN")));
        assert_eq!(sql_expr!(val("Rust")), (Value("Rust")));

        // assert_eq!(
        //     sql_expr!("", val(21), val("str"), val(vec![902, 123, 731])),
        //     (
        //         Expr(""),
        //         Value(21),
        //         Value("str"),
        //         Value(vec![902, 123, 731])
        //     )
        // );
        // assert_eq!(
        //     sql_expr!(expr("IN"), val(21), val("str"), val(vec![902, 123, 731])),
        //     (
        //         Expr("IN"),
        //         Value(21),
        //         Value("str"),
        //         Value(vec![902, 123, 731])
        //     )
        // );
        // assert_eq!(sql_expr!(val("Rust")), (Value("Rust")));
    }

    #[test]
    fn cast() {
        let cast_type = "INTEGER";
        let val = 8923;
        let expression = "Rust";

        assert_eq!(
            cast!(expr(expression) AS cast_type),
            (
                Expr("CAST ("),
                Expr(expression),
                Expr(" AS "),
                Expr(cast_type),
                Expr(")")
            )
        );

        assert_eq!(
            cast!(expr(if false { expression } else { "DATE::NOW()" }) AS cast_type),
            (
                Expr("CAST ("),
                Expr("DATE::NOW()"),
                Expr(" AS "),
                Expr(cast_type),
                Expr(")")
            )
        );

        assert_eq!(
            cast!(val(if false { val } else { 102 }) AS "TEXT"),
            (
                Expr("CAST ("),
                Value(102),
                Expr(" AS "),
                Expr("TEXT"),
                Expr(")")
            )
        );

        assert_eq!(
            cast!(val(val) AS cast_type),
            (
                Expr("CAST ("),
                Value(val),
                Expr(" AS "),
                Expr(cast_type),
                Expr(")")
            )
        );
    }
}
