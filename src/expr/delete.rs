use super::separator_optional;
use crate::WriteSql;

/// Writes a `DELETE FROM` command into the sql command buffer.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::delete;
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// delete(&mut sql);
///
/// assert_eq!(sql.as_command(), "DELETE FROM");
/// # Ok(())
/// # }
/// ```
pub fn delete<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("DELETE FROM");
}

/// Writes a `DELETE FROM <table>` clause to start a delete command with a
/// table.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::{delete_from};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// delete_from(&mut sql, "user");
///
/// assert_eq!(sql.as_command(), "DELETE FROM user");
/// # Ok(())
/// # }
/// ```
pub fn delete_from<Sql, Arg>(sql: &mut Sql, table: &str)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("DELETE FROM ");
    sql.push_cmd(table);
}

/// Writes a `USING` clause for specifying additional tables in the delete
/// clause.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::{delete_using};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// delete_using(&mut sql);
///
/// assert_eq!(sql.as_command(), "USING");
/// # Ok(())
/// # }
/// ```
pub fn delete_using<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("USING");
}

/// Writes a `USING <table>` clause with a iterator of additional tables
/// to be included in the `WHERE` clause of a `DELETE` command.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::{delete_using_iter};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// delete_using_iter(&mut sql, ["user", "access_history"]);
///
/// assert_eq!(sql.as_command(), "USING user, access_history");
/// # Ok(())
/// # }
/// ```
pub fn delete_using_iter<'tbl, Sql, Arg, I>(sql: &mut Sql, tables: I)
where
    Sql: WriteSql<Arg>,
    I: IntoIterator<Item = &'tbl str>,
{
    separator_optional(sql);
    sql.push_cmd("USING");

    let mut tbls = tables.into_iter();
    if let Some(first) = tbls.next() {
        sql.push_cmd(" ");
        sql.push_cmd(first);
    }

    for table in tbls {
        sql.push_cmd(", ");
        sql.push_cmd(table);
    }
}
