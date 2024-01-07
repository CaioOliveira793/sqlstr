use super::separator_optional;
use crate::WriteSql;

/// Write a `UPDATE` command into the sql buffer.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr, expr::update};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// update(&mut sql);
///
/// assert_eq!(sql.as_command(), "UPDATE");
/// # Ok(())
/// # }
/// ```
pub fn update<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("UPDATE");
}

/// Write a `UPDATE <table>` command with a table into the sql buffer.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr, expr::update_table};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// update_table(&mut sql, "user");
///
/// assert_eq!(sql.as_command(), "UPDATE user");
/// # Ok(())
/// # }
/// ```
pub fn update_table<Sql, Arg>(sql: &mut Sql, table: &str)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("UPDATE ");
    sql.push_cmd(table);
}

/// Write a `UPDATE <table> AS <alias>` command with a table and an
/// alias into the sql buffer.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr, expr::update_table_as};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// update_table_as(&mut sql, "user", "u");
///
/// assert_eq!(sql.as_command(), "UPDATE user AS u");
/// # Ok(())
/// # }
/// ```
pub fn update_table_as<Sql, Arg>(sql: &mut Sql, table: &str, alias: &str)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("UPDATE ");
    sql.push_cmd(table);
    sql.push_cmd(" AS ");
    sql.push_cmd(alias);
}

/// Write a `SET` expression of a update clause.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr, expr::set_update};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// set_update(&mut sql);
///
/// assert_eq!(sql.as_command(), "SET");
/// # Ok(())
/// # }
/// ```
pub fn set_update<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("SET");
}

/// Write a `<column> =` expression for setting the column value of a update clause.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr, expr::set_column};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// set_column(&mut sql, "name");
///
/// assert_eq!(sql.as_command(), "name =");
/// # Ok(())
/// # }
/// ```
pub fn set_column<Sql, Arg>(sql: &mut Sql, column: &str)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd(column);
    sql.push_cmd(" =");
}

/// Write a `(<column>, ...) =` expression for setting the tuple value of
/// a update clause.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr, expr::set_tuple};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// set_tuple(&mut sql, ["name", "birthdate", "email"]);
///
/// assert_eq!(sql.as_command(), "(name, birthdate, email) =");
/// # Ok(())
/// # }
/// ```
pub fn set_tuple<'t, Sql, Arg, I>(sql: &mut Sql, tuple: I)
where
    Sql: WriteSql<Arg>,
    I: IntoIterator<Item = &'t str>,
{
    separator_optional(sql);
    sql.push_cmd("(");

    let mut tpl = tuple.into_iter();
    if let Some(first) = tpl.next() {
        sql.push_cmd(first);
    }
    for column in tpl {
        sql.push_cmd(", ");
        sql.push_cmd(column);
    }

    sql.push_cmd(") =");
}
