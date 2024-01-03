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

/// Write a `UPDATE <table> [ AS <alias> ]` command with a table and a
/// optional alias into the sql buffer.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr, expr::update_table};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// update_table(&mut sql, "user", Some("u"));
///
/// assert_eq!(sql.as_command(), "UPDATE user AS u");
/// # Ok(())
/// # }
/// ```
pub fn update_table<Sql, Arg>(sql: &mut Sql, table: &str, alias: Option<&str>)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("UPDATE ");
    sql.push_cmd(table);

    if let Some(alias) = alias {
        sql.push_cmd(" AS ");
        sql.push_cmd(alias);
    }
}

/// Write a `SET <column> =` expression for setting the column value of a
/// update clause.
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
/// assert_eq!(sql.as_command(), "SET name =");
/// # Ok(())
/// # }
/// ```
pub fn set_column<Sql, Arg>(sql: &mut Sql, column: &str)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("SET ");
    sql.push_cmd(column);
    sql.push_cmd(" =");
}

/// Write a `SET (<column>, ...) =` expression for setting the tuple value of
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
/// assert_eq!(sql.as_command(), "SET (name, birthdate, email) =");
/// # Ok(())
/// # }
/// ```
pub fn set_tuple<'t, Sql, Arg, I>(sql: &mut Sql, tuple: I)
where
    Sql: WriteSql<Arg>,
    I: IntoIterator<Item = &'t str>,
{
    separator_optional(sql);
    sql.push_cmd("SET (");

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
