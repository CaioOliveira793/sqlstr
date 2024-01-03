use super::separator_optional;
use crate::WriteSql;

/// SELECT
///
/// The select command retrieves rows from zero or more tables.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr, expr::select};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// select(&mut sql);
///
/// assert_eq!(sql.as_command(), "SELECT");
/// # Ok(())
/// # }
/// ```
pub fn select<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("SELECT");
}

/// SELECT ALL
///
/// The select query will return all the candidate rows, including duplicates (database default).
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr, expr::select_all};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// select_all(&mut sql);
///
/// assert_eq!(sql.as_command(), "SELECT ALL");
/// # Ok(())
/// # }
/// ```
pub fn select_all<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("SELECT ALL");
}

/// SELECT DISTINCT
///
/// The select query will return only the distinct cantidate rows, eliminating duplicates.
///
/// # Example
///
/// ```
/// # use sqlstr::{SqlCommand, Void, SqlExpr, expr::select_distinct};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// select_distinct(&mut sql);
///
/// assert_eq!(sql.as_command(), "SELECT DISTINCT");
/// # Ok(())
/// # }
/// ```
pub fn select_distinct<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("SELECT DISTINCT");
}

#[macro_export]
macro_rules! static_select {
    (SELECT) => {
        "SELECT"
    };
    (SELECT ALL) => {
        "SELECT ALL"
    };
    (SELECT DISTINCT) => {
        "SELECT DISTINCT"
    };
}

pub use static_select;

#[cfg(test)]
mod test {
    #[test]
    fn static_select_macro() {
        assert_eq!(static_select!(SELECT), "SELECT");

        assert_eq!(static_select!(SELECT ALL), "SELECT ALL");

        assert_eq!(static_select!(SELECT DISTINCT), "SELECT DISTINCT");
    }
}
