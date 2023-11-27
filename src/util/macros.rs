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

#[cfg(test)]
mod test {
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
}
