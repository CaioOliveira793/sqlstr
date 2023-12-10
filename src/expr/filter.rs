use crate::WriteSql;

use super::separator_optional;

/// Write a `WHERE` clause in the sql command.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::filter_where};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// filter_where(&mut sql);
///
/// assert_eq!(sql.as_command(), "WHERE");
/// # Ok(())
/// # }
/// ```
pub fn filter_where<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("WHERE");
}

/// Write a `HAVING` clause in the sql command.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::filter_having};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// filter_having(&mut sql);
///
/// assert_eq!(sql.as_command(), "HAVING");
/// # Ok(())
/// # }
/// ```
pub fn filter_having<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("HAVING");
}
