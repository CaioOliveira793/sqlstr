mod macros;

pub use macros::*;

use crate::command::{ArgumentBuffer, SqlExpr, WriteSql};

/// Puts an item separator `", "` into the command buffer if the sql command does
/// not ends with one.
///
/// # Example
///
/// The separator is only added if not preceded at the end of the sql command.
///
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr, util::item_separator_option};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT ");
/// sql.push_value(47);
/// sql.push_cmd(",   ");
/// assert_eq!(sql.as_command(), "SELECT $1,   ");
///
/// item_separator_option(&mut sql);
/// assert_eq!(sql.as_command(), "SELECT $1,   ");
/// # Ok(())
/// # }
/// ```
///
/// In case the item separator (`','`) is present, only a space (`' '`) is added.
///
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr, util::item_separator_option};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT ");
/// sql.push_value(91);
/// sql.push_cmd(",");
/// assert_eq!(sql.as_command(), "SELECT $1,");
///
/// item_separator_option(&mut sql);
/// assert_eq!(sql.as_command(), "SELECT $1, ");
/// # Ok(())
/// # }
/// ```
///
/// When the sql buffer does not end with an item separator, one is added.
///
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr, util::item_separator_option};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT ");
/// sql.push_value(-1);
/// item_separator_option(&mut sql);
/// sql.push_value("Rust");
///
/// assert_eq!(sql.as_command(), "SELECT $1, $2");
/// # Ok(())
/// # }
/// ```
pub fn item_separator_option<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    // "SELECT $1,"
    if sql.as_command().ends_with(',') {
        sql.push_cmd(" ");
        return;
    }

    // "SELECT $1,   "
    match sql.as_command().chars().rev().find(|ch| *ch != ' ') {
        Some(',') | None => {}
        Some(_) => sql.push_cmd(","),
    }
}

/// Puts an item separator `", "` into the command buffer
///
/// # Example
///
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr, util::item_separator};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT ");
/// sql.push_value(47)?;
/// item_separator(&mut sql);
/// sql.push_value(53)?;
///
/// assert_eq!(sql.as_command(), "SELECT $1, $2");
/// # Ok(())
/// # }
/// ```
pub fn item_separator<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    sql.push_cmd(", ");
}

/// Puts a separator `" "` into the command buffer if it does't already have.
///
/// # Example
///
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr, util::separator_option};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT");
/// separator_option(&mut sql);
/// sql.push_value(47)?;
///
/// assert_eq!(sql.as_command(), "SELECT $1");
///
/// sql.push_cmd(", ");
/// separator_option(&mut sql);
/// sql.push_value(53)?;
///
/// assert_eq!(sql.as_command(), "SELECT $1, $2");
/// # Ok(())
/// # }
/// ```
pub fn separator_option<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    if !sql.as_command().ends_with(' ') {
        sql.push_cmd(" ");
    }
}

/// Puts a separator `" "` into the command buffer
///
/// # Example
///
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr, util::separator};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT");
/// separator(&mut sql);
/// sql.push_value(47)?;
///
/// assert_eq!(sql.as_command(), "SELECT $1");
/// # Ok(())
/// # }
/// ```
pub fn separator<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    sql.push_cmd(" ");
}

/// **CAST** Command
///
/// A cast specifies how to perform a conversion between two data types.
///
/// # Example
///
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr, util::cast};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// cast(&mut sql, SqlExpr::Value("9123"), "NUMERIC")?;
///
/// assert_eq!(sql.as_command(), "CAST ($1 AS NUMERIC)");
/// # Ok(())
/// # }
/// ```
pub fn cast<Sql, Arg, T>(
    sql: &mut Sql,
    expr: SqlExpr<'_, T>,
    typ: &'_ str,
) -> Result<(), Arg::Error>
where
    Sql: WriteSql<Arg>,
    Arg: ArgumentBuffer<T>,
{
    sql.push_cmd("CAST (");
    sql.push_expr(expr)?;
    sql.push_cmd(" AS ");
    sql.push_cmd(typ);
    sql.push_cmd(")");
    Ok(())
}
