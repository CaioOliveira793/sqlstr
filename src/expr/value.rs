use super::{item_separator_optional, separator_optional};
use crate::{ArgumentBuffer, WriteSql};

/// Write a `VALUES` clause to start a list of expressions to be used in the
/// `INSERT` command.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::{values, insert_into, columns_iter, write_iter};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// values(&mut sql);
///
/// assert_eq!(sql.as_command(), "VALUES");
/// # Ok(())
/// # }
/// ```
pub fn values<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("VALUES");
}

/// Write a list of values into the sql command.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::{write_iter, select, separator};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// select(&mut sql);
/// separator(&mut sql);
/// write_iter(&mut sql, [0, -100, 47, 69])?;
///
/// assert_eq!(sql.as_command(), "SELECT $1, $2, $3, $4");
/// # Ok(())
/// # }
/// ```
pub fn write_iter<Sql, Arg, I, T>(sql: &mut Sql, values: I) -> Result<(), Arg::Error>
where
    Sql: WriteSql<Arg>,
    Arg: ArgumentBuffer<T>,
    I: IntoIterator<Item = T>,
{
    item_separator_optional(sql);

    let mut val_iter = values.into_iter();
    if let Some(val) = val_iter.next() {
        sql.push_value(val)?;
    }

    for val in val_iter {
        sql.push_cmd(", ");
        sql.push_value(val)?;
    }

    Ok(())
}

#[macro_export]
macro_rules! write_variadic {
    (ArgumentBufferError = $argbuf_error_type:ty; $sql:expr, $value1:expr) => {{
        let sql = $sql;
        item_separator_optional(sql);
        sql.push_value($value1)
    }};

    (ArgumentBufferError = $argbuf_error_type:ty; $sql:expr, $value1:expr, $($valuex:expr),*) => {{
        let mut value_writter = || -> Result<(), $argbuf_error_type> {
            let sql = $sql;
            item_separator_optional(sql);
            sql.push_value($value1)?;

            $(
                sql.push_cmd(", ");
                sql.push_value($valuex)?;
            )*

            Ok(())
        };
        value_writter()
    }};
    ($sql:expr, $value1:expr, $($valuex:expr),*) => {{
        let mut value_writter = || {
            let sql = $sql;
            item_separator_optional(sql);
            sql.push_value($value1)?;

            $(
                sql.push_cmd(", ");
                sql.push_value($valuex)?;
            )*

            Ok(())
        };
        value_writter()
    }};
}

pub use write_variadic;

#[cfg(test)]
mod test {
    use alloc::string::String;

    use super::*;
    use crate::{
        expr::{select, separator},
        test::{display_iter, TestArgs},
        SqlCommand,
    };

    #[test]
    fn select_values_iter() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        select(&mut sql);
        separator(&mut sql);
        write_iter(&mut sql, [10, -100, 0, -999]).unwrap();

        assert_eq!(sql.command, "SELECT $1, $2, $3, $4");
        assert_eq!(sql.arguments.as_str(), "10;-100;0;-999;");
    }

    #[test]
    fn select_values_iter_multiple_times() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        select(&mut sql);
        separator(&mut sql);
        write_iter(&mut sql, [10, -100, 0, -999]).unwrap();
        write_iter(&mut sql, ["r", "u", "s", "t"]).unwrap();

        assert_eq!(sql.command, "SELECT $1, $2, $3, $4, $5, $6, $7, $8");
        assert_eq!(sql.arguments.as_str(), "10;-100;0;-999;r;u;s;t;");
    }

    #[test]
    fn values_macro_single() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        select(&mut sql);
        separator(&mut sql);

        write_variadic!(ArgumentBufferError = core::fmt::Error; &mut sql, true).unwrap();

        assert_eq!(sql.as_command(), "SELECT $1");
        assert_eq!(sql.arguments.as_str(), "true;");
    }

    #[test]
    fn values_macro_variadic() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        select(&mut sql);
        separator(&mut sql);

        write_variadic!(
            ArgumentBufferError = core::fmt::Error;
            &mut sql,
            "str",
            false,
            String::from("string"),
            10,
            display_iter(&[120, 360, 0]).unwrap()
        )
        .unwrap();

        assert_eq!(sql.as_command(), "SELECT $1, $2, $3, $4, $5");
        assert_eq!(sql.arguments.as_str(), "str;false;string;10;[120,360,0];");
    }
}
