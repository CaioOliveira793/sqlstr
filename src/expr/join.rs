use crate::WriteSql;

pub enum JoinType {
    Cross,
    Inner,
    Left,
    Right,
    Full,
}

impl JoinType {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Cross => "CROSS",
            Self::Inner => "INNER",
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
            Self::Full => "FULL",
        }
    }
}

/// Writes a `CROSS JOIN <table>` clause
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr};
/// # use sqlstr::expr::{cross_join};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// cross_join(&mut sql, "post");
///
/// assert_eq!(sql.as_command(), "CROSS JOIN post");
/// # Ok(())
/// # }
/// ```
pub fn cross_join<Sql, Arg>(sql: &mut Sql, table: &str)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("CROSS JOIN ");
    sql.push_cmd(table);
}

/// Starts a `JOIN` clause.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr};
/// # use sqlstr::expr::{join, JoinType};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// join(&mut sql, JoinType::Left, "user", Some("u"));
///
/// assert_eq!(sql.as_command(), "LEFT JOIN user AS u");
/// # Ok(())
/// # }
/// ```
pub fn join<Sql, Arg>(sql: &mut Sql, typ: JoinType, table: &str, alias: Option<&str>)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd(typ.as_str());
    sql.push_cmd(" JOIN ");
    sql.push_cmd(table);
    if let Some(alias) = alias {
        sql.push_cmd(" AS ");
        sql.push_cmd(alias);
    }
}

/// Starts a join condition.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr};
/// # use sqlstr::expr::{join_on, join, JoinType};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// join(&mut sql, JoinType::Inner, "customer", None);
/// join_on(&mut sql);
///
/// assert_eq!(sql.as_command(), "INNER JOIN customer ON");
/// # Ok(())
/// # }
/// ```
pub fn join_on<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("ON");
}

/// Writes a join condition with the `USING` form.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr};
/// # use sqlstr::expr::{join_using, join, JoinType};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// join(&mut sql, JoinType::Right, "customer", Some("c"));
/// join_using(&mut sql, ["attendant_id"]);
///
/// assert_eq!(sql.as_command(), "RIGHT JOIN customer AS c USING (attendant_id)");
/// # Ok(())
/// # }
/// ```
pub fn join_using<'t, Sql, Arg, I>(sql: &mut Sql, columns: I)
where
    Sql: WriteSql<Arg>,
    I: IntoIterator<Item = &'t str>,
{
    separator_optional(sql);
    sql.push_cmd("USING (");

    let mut tbls = columns.into_iter();
    if let Some(tbl) = tbls.next() {
        sql.push_cmd(tbl);
    }

    for tbl in tbls {
        sql.push_cmd(", ");
        sql.push_cmd(tbl);
    }
    sql.push_cmd(")");
}

/// Comma separated list of shared column names
#[macro_export]
macro_rules! static_join_using {
    ($first:literal) => {
        concat!("USING (", $first, ")")
    };

    ($first:literal, $($column:literal),* $(,)?) => {
        concat!("USING (", $first, $(", ", $column),*, ")")
    };
}

#[macro_export]
macro_rules! static_join {
    (CROSS $table:literal) => {
        concat!("CROSS JOIN ", $table)
    };

    (INNER $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "INNER JOIN ",
            $table,
            " ON ",
            $crate::expr::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (INNER $table:literal ON NOT $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "INNER JOIN ",
            $table,
            " ON NOT ",
            $crate::expr::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (INNER $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "INNER JOIN ",
            $table,
            " ",
            $crate::expr::static_join_using!($first, $($column),*),
        )
    };

    (LEFT $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "LEFT JOIN ",
            $table,
            " ON ",
            $crate::expr::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (LEFT $table:literal ON NOT $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "LEFT JOIN ",
            $table,
            " ON NOT ",
            $crate::expr::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (LEFT $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "LEFT JOIN ",
            $table,
            " ",
            $crate::expr::static_join_using!($first, $($column),*),
        )
    };

    (RIGHT $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "RIGHT JOIN ",
            $table,
            " ON ",
            $crate::expr::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (RIGHT $table:literal ON NOT $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "RIGHT JOIN ",
            $table,
            " ON NOT ",
            $crate::expr::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (RIGHT $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "RIGHT JOIN ",
            $table,
            " ",
            $crate::expr::static_join_using!($first, $($column),*),
        )
    };

    (FULL $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "FULL JOIN ",
            $table,
            " ON ",
            $crate::expr::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (FULL $table:literal ON NOT $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "FULL JOIN ",
            $table,
            " ON NOT ",
            $crate::expr::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (FULL $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "FULL JOIN ",
            $table,
            " ",
            $crate::expr::static_join_using!($first, $($column),*)
        )
    };
}

pub use static_join;
pub use static_join_using;

use super::separator_optional;

#[cfg(test)]
mod test {
    use crate::{expr::join_using, test::TestArgs, SqlCommand};

    #[test]
    fn join_using_single_column() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();
        join_using(&mut sql, ["id"]);
        assert_eq!(sql.as_command(), "USING (id)");
    }

    #[test]
    fn join_using_multiple_columns() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();
        join_using(&mut sql, ["id", "customer_id"]);
        assert_eq!(sql.as_command(), "USING (id, customer_id)");

        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();
        join_using(&mut sql, ["id", "sale_id", "customer_id"]);
        assert_eq!(sql.as_command(), "USING (id, sale_id, customer_id)");
    }

    #[test]
    fn static_using_macro() {
        assert_eq!(static_join_using!("id"), "USING (id)");
        assert_eq!(
            static_join_using!("id", "customer_id"),
            "USING (id, customer_id)"
        );
        assert_eq!(
            static_join_using!("id", "sale_id", "customer_id",),
            "USING (id, sale_id, customer_id)"
        );
    }

    #[test]
    fn static_join_macro() {
        assert_eq!(static_join!(CROSS "user"), "CROSS JOIN user");

        assert_eq!(
            static_join!(INNER "user" USING ("id", "department")),
            "INNER JOIN user USING (id, department)"
        );
        assert_eq!(
            static_join!(INNER "user" ON "user.id" = "access_history.user_id"),
            "INNER JOIN user ON user.id = access_history.user_id"
        );
        assert_eq!(
            static_join!(INNER "user" ON NOT "user.id" = "access_history.user_id" AND_NOT "user.updated" > "access_history.created"),
            "INNER JOIN user ON NOT user.id = access_history.user_id AND NOT user.updated > access_history.created"
        );

        assert_eq!(
            static_join!(LEFT "user" USING ("id", "department")),
            "LEFT JOIN user USING (id, department)"
        );
        assert_eq!(
            static_join!(LEFT "user" ON "user.id" = "access_history.user_id"),
            "LEFT JOIN user ON user.id = access_history.user_id"
        );
        assert_eq!(
            static_join!(LEFT "user" ON NOT "user.id" = "access_history.user_id"),
            "LEFT JOIN user ON NOT user.id = access_history.user_id"
        );
        assert_eq!(
            static_join!(LEFT "user" ON "user.id" = "access_history.user_id" OR "user.updated" <= "access_history.created"),
            "LEFT JOIN user ON user.id = access_history.user_id OR user.updated <= access_history.created"
        );

        assert_eq!(
            static_join!(RIGHT "user" USING ("id", "department")),
            "RIGHT JOIN user USING (id, department)"
        );
        assert_eq!(
            static_join!(RIGHT "user" ON "user.id" = "access_history.user_id"),
            "RIGHT JOIN user ON user.id = access_history.user_id"
        );
        assert_eq!(
            static_join!(RIGHT "user" ON NOT "user.id" = "access_history.user_id" AND "user.updated" < "access_history.created"),
            "RIGHT JOIN user ON NOT user.id = access_history.user_id AND user.updated < access_history.created"
        );
        assert_eq!(
            static_join!(RIGHT "user" ON "user.id" = "access_history.user_id" AND_NOT "user.updated" >= "access_history.created"),
            "RIGHT JOIN user ON user.id = access_history.user_id AND NOT user.updated >= access_history.created"
        );

        assert_eq!(
            static_join!(FULL "user" USING ("id", "department")),
            "FULL JOIN user USING (id, department)"
        );
        assert_eq!(
            static_join!(FULL "user" ON "user.id" != "access_history.user_id"),
            "FULL JOIN user ON user.id <> access_history.user_id"
        );
        assert_eq!(
            static_join!(FULL "user" ON "user.id" = "access_history.user_id" OR "user.updated" < "access_history.created"),
            "FULL JOIN user ON user.id = access_history.user_id OR user.updated < access_history.created"
        );
        assert_eq!(
            static_join!(FULL "user" ON NOT "user.id" != "access_history.user_id" OR_NOT "user.updated" < "access_history.created"),
            "FULL JOIN user ON NOT user.id <> access_history.user_id OR NOT user.updated < access_history.created"
        );
    }
}
