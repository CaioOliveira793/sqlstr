use super::separator_optional;
use crate::WriteSql;

#[derive(Clone, Default, PartialEq, Eq)]
pub enum OrderByOrd<'expr> {
    Asc,
    Desc,
    Using(&'expr str),
    /// Detabase default
    #[default]
    Default,
}

#[derive(Clone, Default, PartialEq, Eq)]
pub enum OrderByNulls {
    First,
    Last,
    /// Database default
    #[default]
    Default,
}

/// Writes a `ORDER BY` clause.
///
/// # Example
///
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr};
/// # use squeal_builder::expr::{select, order_by, OrderByOrd, OrderByNulls};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// order_by(&mut sql, [("age", OrderByOrd::Asc, OrderByNulls::Last)]);
///
/// assert_eq!(sql.as_command(), "ORDER BY age ASC NULLS LAST");
/// # Ok(())
/// # }
/// ```
///
/// In case the defaults from the database must be used, use the `Default` variant from
/// the [OrderByOrd] and [OrderByNulls].
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr};
/// # use squeal_builder::expr::{select, order_by, OrderByOrd, OrderByNulls};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// order_by(&mut sql, [("age", OrderByOrd::Default, OrderByNulls::Default)]);
///
/// assert_eq!(sql.as_command(), "ORDER BY age");
/// # Ok(())
/// # }
/// ```
pub fn order_by<'col, 'ord_expr, Sql, Arg, I>(sql: &mut Sql, order_exprs: I)
where
    Sql: WriteSql<Arg>,
    I: IntoIterator<Item = (&'col str, OrderByOrd<'ord_expr>, OrderByNulls)>,
{
    separator_optional(sql);
    sql.push_cmd("ORDER BY");

    let mut orderexpr = order_exprs.into_iter();
    if let Some(expr) = orderexpr.next() {
        sql.push_cmd(" ");
        order_by_expr(sql, expr.0, expr.1, expr.2);
    }

    for expr in orderexpr {
        sql.push_cmd(", ");
        order_by_expr(sql, expr.0, expr.1, expr.2);
    }
}

/// Write a `ORDER BY` order expression.
///
/// # Example
///
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr};
/// # use squeal_builder::expr::{select, order_by_expr, OrderByOrd, OrderByNulls};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// order_by_expr(&mut sql, "age", OrderByOrd::Asc, OrderByNulls::Last);
///
/// assert_eq!(sql.as_command(), "age ASC NULLS LAST");
/// # Ok(())
/// # }
/// ```
pub fn order_by_expr<Sql, Arg>(sql: &mut Sql, column: &str, order: OrderByOrd, nulls: OrderByNulls)
where
    Sql: WriteSql<Arg>,
{
    sql.push_cmd(column);

    match order {
        OrderByOrd::Desc => sql.push_cmd(" DESC"),
        OrderByOrd::Asc => sql.push_cmd(" ASC"),
        OrderByOrd::Using(op) => {
            sql.push_cmd(" USING ");
            sql.push_cmd(op);
        }
        OrderByOrd::Default => {}
    }

    match nulls {
        OrderByNulls::First => sql.push_cmd(" NULLS FIRST"),
        OrderByNulls::Last => sql.push_cmd(" NULLS LAST"),
        OrderByNulls::Default => {}
    }
}

#[allow(unused_macros)]
macro_rules! order_by_criteria {
    () => {
        ""
    };
    (ASC) => {
        "ASC"
    };
    (DESC) => {
        "DESC"
    };
    (USING $op:tt) => {
        concat!("USING ", $crate::expr::static_comparison!($op))
    };

    (NULLS FIRST) => {
        "NULLS FIRST"
    };
    (NULLS LAST) => {
        "NULLS LAST"
    };

    (ASC NULLS FIRST) => {
        "ASC NULLS FIRST"
    };
    (DESC NULLS FIRST) => {
        "DESC NULLS FIRST"
    };
    (USING $op:tt NULLS FIRST) => {
        concat!(
            "USING ",
            $crate::expr::static_comparison!($op),
            " NULLS FIRST"
        )
    };

    (ASC NULLS LAST) => {
        "ASC NULLS LAST"
    };
    (DESC NULLS LAST) => {
        "DESC NULLS LAST"
    };
    (USING $op:tt NULLS LAST) => {
        concat!(
            "USING ",
            $crate::expr::static_comparison!($op),
            " NULLS LAST"
        )
    };
}

#[allow(unused_imports)]
pub(super) use order_by_criteria;

#[allow(unused_macros)]
macro_rules! nested_order_by {
    ($col:literal$(,)? $($colx:literal $(ord($($rest:tt)+))?),*) => {
        concat!(
            ", ",
            $col,
			$($crate::expr::nested_order_by!($colx $(ord($($rest)+))?)),*
        )
    };
    ($col:literal ord($($criteria:tt)+)$(,)? $($colx:literal $(ord($($rest:tt)+))?),*) => {
        concat!(
            ", ",
            $col,
			" ",
    		$crate::expr::order_by_criteria!($($criteria)+),
    		$($crate::expr::nested_order_by!($colx $(ord($($rest)+))?)),*
        )
    };
}

#[allow(unused_imports)]
pub(super) use nested_order_by;

#[macro_export]
macro_rules! static_order_by {
    ($col:literal$(,)? $($colx:literal $(ord($($rest:tt)+))?),*) => {
        concat!(
            "ORDER BY ",
            $col,
			$($crate::expr::nested_order_by!($colx $(ord($($rest)+))?)),*
        )
    };
    ($col:literal ord($($criteria:tt)+)$(,)? $($colx:literal $(ord($($rest:tt)+))?),*) => {
        concat!(
            "ORDER BY ",
            $col,
			" ",
    		$crate::expr::order_by_criteria!($($criteria)+),
    		$($crate::expr::nested_order_by!($colx $(ord($($rest)+))?)),*
        )
    };
}

pub use static_order_by;

#[cfg(test)]
mod test {
    #[test]
    fn static_order_by_macro() {
        assert_eq!(static_order_by!("id"), "ORDER BY id");
        assert_eq!(static_order_by!("id" ord(ASC)), "ORDER BY id ASC");
        assert_eq!(static_order_by!("id" ord(DESC)), "ORDER BY id DESC");
        assert_eq!(static_order_by!("id" ord(USING >)), "ORDER BY id USING >");

        assert_eq!(
            static_order_by!("id" ord(NULLS FIRST)),
            "ORDER BY id NULLS FIRST"
        );
        assert_eq!(
            static_order_by!("id" ord(ASC NULLS FIRST)),
            "ORDER BY id ASC NULLS FIRST"
        );
        assert_eq!(
            static_order_by!("id" ord(DESC NULLS FIRST)),
            "ORDER BY id DESC NULLS FIRST"
        );
        assert_eq!(
            static_order_by!("id" ord(USING > NULLS FIRST)),
            "ORDER BY id USING > NULLS FIRST"
        );

        assert_eq!(
            static_order_by!("id" ord(NULLS LAST)),
            "ORDER BY id NULLS LAST"
        );
        assert_eq!(
            static_order_by!("id" ord(ASC NULLS LAST)),
            "ORDER BY id ASC NULLS LAST"
        );
        assert_eq!(
            static_order_by!("id" ord(DESC NULLS LAST)),
            "ORDER BY id DESC NULLS LAST"
        );
        assert_eq!(
            static_order_by!("id" ord(USING > NULLS LAST)),
            "ORDER BY id USING > NULLS LAST"
        );

        assert_eq!(
            static_order_by!("id", "access_id"),
            "ORDER BY id, access_id"
        );
        assert_eq!(
            static_order_by!("id" ord(ASC), "access_id"),
            "ORDER BY id ASC, access_id"
        );
        assert_eq!(
            static_order_by!("id" ord(DESC), "access_id"),
            "ORDER BY id DESC, access_id"
        );
        assert_eq!(
            static_order_by!("id" ord(USING >), "access_id"),
            "ORDER BY id USING >, access_id"
        );

        assert_eq!(
            static_order_by!("id", "access_id" ord(ASC)),
            "ORDER BY id, access_id ASC"
        );
        assert_eq!(
            static_order_by!("id" ord(ASC), "access_id" ord(ASC)),
            "ORDER BY id ASC, access_id ASC"
        );
        assert_eq!(
            static_order_by!("id" ord(DESC), "access_id" ord(ASC)),
            "ORDER BY id DESC, access_id ASC"
        );
        assert_eq!(
            static_order_by!("id" ord(USING >), "access_id" ord(ASC)),
            "ORDER BY id USING >, access_id ASC"
        );

        assert_eq!(
            static_order_by!("id", "access_id" ord(DESC)),
            "ORDER BY id, access_id DESC"
        );
        assert_eq!(
            static_order_by!("id" ord(ASC), "access_id" ord(DESC)),
            "ORDER BY id ASC, access_id DESC"
        );
        assert_eq!(
            static_order_by!("id" ord(DESC), "access_id" ord(DESC)),
            "ORDER BY id DESC, access_id DESC"
        );
        assert_eq!(
            static_order_by!("id" ord(USING >), "access_id" ord(DESC)),
            "ORDER BY id USING >, access_id DESC"
        );

        assert_eq!(
            static_order_by!("id", "access_id" ord(USING <)),
            "ORDER BY id, access_id USING <"
        );
        assert_eq!(
            static_order_by!("id" ord(ASC), "access_id" ord(USING < NULLS FIRST)),
            "ORDER BY id ASC, access_id USING < NULLS FIRST"
        );
        assert_eq!(
            static_order_by!("id" ord(DESC), "access_id" ord(USING <)),
            "ORDER BY id DESC, access_id USING <"
        );
        assert_eq!(
            static_order_by!("id" ord(USING >), "access_id" ord(USING < NULLS LAST)),
            "ORDER BY id USING >, access_id USING < NULLS LAST"
        );

        // NULLS FIRST
        assert_eq!(
            static_order_by!("id" ord(NULLS FIRST), "access_id" ord(ASC)),
            "ORDER BY id NULLS FIRST, access_id ASC"
        );
        assert_eq!(
            static_order_by!("id" ord(ASC NULLS FIRST), "access_id" ord(DESC)),
            "ORDER BY id ASC NULLS FIRST, access_id DESC"
        );
        assert_eq!(
            static_order_by!("id" ord(DESC NULLS FIRST), "access_id" ord(USING > NULLS LAST)),
            "ORDER BY id DESC NULLS FIRST, access_id USING > NULLS LAST"
        );
        assert_eq!(
            static_order_by!("id" ord(USING > NULLS LAST), "access_id" ord(ASC)),
            "ORDER BY id USING > NULLS LAST, access_id ASC"
        );

        assert_eq!(
            static_order_by!("x" ord(DESC), "y" ord(DESC NULLS FIRST), "z" ord(ASC), "a" ord(USING > NULLS LAST)),
            "ORDER BY x DESC, y DESC NULLS FIRST, z ASC, a USING > NULLS LAST"
        );
    }
}
