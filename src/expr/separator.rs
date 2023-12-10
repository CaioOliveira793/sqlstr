use crate::WriteSql;

/// Puts an item separator `", "` into the command buffer if the sql command does
/// not ends with one.
///
/// # Example
///
/// The separator is only added if not preceded at the end of the sql command.
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::item_separator_optional};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT ");
/// sql.push_value(47);
/// sql.push_cmd(",   ");
/// assert_eq!(sql.as_command(), "SELECT $1,   ");
///
/// item_separator_optional(&mut sql);
/// assert_eq!(sql.as_command(), "SELECT $1,   ");
/// # Ok(())
/// # }
/// ```
///
/// In case the item separator (`','`) is present, only a space (`' '`) is added.
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::item_separator_optional};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT ");
/// sql.push_value(91);
/// sql.push_cmd(",");
/// assert_eq!(sql.as_command(), "SELECT $1,");
///
/// item_separator_optional(&mut sql);
/// assert_eq!(sql.as_command(), "SELECT $1, ");
/// # Ok(())
/// # }
/// ```
///
/// When the sql buffer does not end with an item separator, one is added.
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::item_separator_optional};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT ");
/// sql.push_value(-1);
/// item_separator_optional(&mut sql);
/// sql.push_value("Rust");
///
/// assert_eq!(sql.as_command(), "SELECT $1, $2");
/// # Ok(())
/// # }
/// ```
///
/// Only if a argument value is present, a separator is placed.
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::item_separator_optional};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT ");
/// item_separator_optional(&mut sql);
///
/// assert_eq!(sql.as_command(), "SELECT ");
///
/// sql.push_value(-1);
/// item_separator_optional(&mut sql);
/// sql.push_value("Rust");
///
/// assert_eq!(sql.as_command(), "SELECT $1, $2");
/// # Ok(())
/// # }
/// ```
pub fn item_separator_optional<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    if sql.as_command().is_empty() {
        return;
    }

    // "SELECT $1,"
    if sql.as_command().ends_with(',') {
        sql.push_cmd(" ");
        return;
    }

    // "SELECT $1   "
    // FIXME: prevent match "$   "
    for ch in sql.as_command().trim_end_matches(' ').chars().rev() {
        match ch {
            '0'..='9' => continue,
            '$' => {
                sql.push_cmd(", ");
                return;
            }
            _ => return,
        }
    }
}

/// Puts an item separator `", "` into the command buffer
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::item_separator};
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
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::separator_optional};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// sql.push_cmd("SELECT");
/// separator_optional(&mut sql);
/// sql.push_value(47)?;
///
/// assert_eq!(sql.as_command(), "SELECT $1");
///
/// sql.push_cmd(", ");
/// separator_optional(&mut sql);
/// sql.push_value(53)?;
///
/// assert_eq!(sql.as_command(), "SELECT $1, $2");
/// # Ok(())
/// # }
/// ```
pub fn separator_optional<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    if sql.as_command().is_empty() || sql.as_command().ends_with('(') {
        return;
    }

    if !sql.as_command().ends_with(' ') {
        sql.push_cmd(" ");
    }
}

/// Puts a separator `" "` into the command buffer
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::separator};
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
