use crate::WriteSql;

use super::separator_optional;

/// Write a `RETURNING` clause to compute the values that will be returned from
/// the query.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::returning};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// returning(&mut sql);
///
/// assert_eq!(sql.as_command(), "RETURNING");
/// # Ok(())
/// # }
/// ```
pub fn returning<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("RETURNING");
}
