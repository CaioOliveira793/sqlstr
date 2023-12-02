/// Create a valid GROUP BY clause as a `&'static str` at compile-time.
///
/// # Example
///
/// ```
/// # use squeal::expr::static_group_by;
/// const GROUP_BY_CLAUSE: &str = static_group_by!("id", "customer_id");
///
/// assert_eq!(GROUP_BY_CLAUSE, "GROUP BY id, customer_id");
/// ```
#[macro_export]
macro_rules! static_group_by {
    ($first:literal$(,)? $($column:literal),* $(,)?) => {
        concat!("GROUP BY ", $first, $(", ", $column),*)
    };
}

pub use static_group_by;

use super::separator_optional;
use crate::WriteSql;

/// Writes a `GROUP BY` clause.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::{group_by, select};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// group_by(&mut sql, ["id", "customer_id"]);
///
/// assert_eq!(sql.as_command(), "GROUP BY id, customer_id");
/// # Ok(())
/// # }
/// ```
pub fn group_by<'col, Sql, Arg, I>(sql: &mut Sql, columns: I)
where
    Sql: WriteSql<Arg>,
    I: IntoIterator<Item = &'col str>,
{
    separator_optional(sql);
    sql.push_cmd("GROUP BY");

    let mut cols = columns.into_iter();
    if let Some(col) = cols.next() {
        sql.push_cmd(" ");
        sql.push_cmd(col);
    }
    for col in cols {
        sql.push_cmd(", ");
        sql.push_cmd(col);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{test::TestArgs, SqlCommand};

    #[test]
    fn group_by_single_column() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        group_by(&mut sql, ["id"]);

        assert_eq!(sql.as_command(), "GROUP BY id");
    }

    #[test]
    fn group_by_no_column() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        group_by(&mut sql, []);

        assert_eq!(sql.as_command(), "GROUP BY");
    }
}
