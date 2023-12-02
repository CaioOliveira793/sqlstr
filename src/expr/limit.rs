use super::separator_optional;
use crate::{ArgumentBuffer, WriteSql};

/// Writes a LIMIT clause into a sql writer.
///
/// LIMIT <count>
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::limit};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// limit(&mut sql, 10);
///
/// assert_eq!(sql.as_command(), "LIMIT $1");
/// # Ok(())
/// # }
/// ```
pub fn limit<Sql, Arg>(sql: &mut Sql, count: usize) -> Result<(), Arg::Error>
where
    Sql: WriteSql<Arg>,
    Arg: ArgumentBuffer<usize>,
{
    separator_optional(sql);
    sql.push_cmd("LIMIT ");
    sql.push_value(count)
}

/// Writes a offset clause into a sql writer.
///
/// OFFSET <start>
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr, expr::offset};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// offset(&mut sql, 30);
///
/// assert_eq!(sql.as_command(), "OFFSET $1");
/// # Ok(())
/// # }
/// ```
pub fn offset<Sql, Arg>(sql: &mut Sql, start: usize) -> Result<(), Arg::Error>
where
    Sql: WriteSql<Arg>,
    Arg: ArgumentBuffer<usize>,
{
    separator_optional(sql);
    sql.push_cmd("OFFSET ");
    sql.push_value(start)
}

#[macro_export]
macro_rules! static_limit {
    (LIMIT $count:literal) => {
        concat!("LIMIT ", $count)
    };
}

pub use static_limit;

#[macro_export]
macro_rules! static_offset {
    (OFFSET $start:literal) => {
        concat!("OFFSET ", $start)
    };
}

pub use static_offset;

#[cfg(test)]
mod test {
    #[test]
    fn static_limit_macro() {
        assert_eq!(static_limit!(LIMIT 15), "LIMIT 15");
        assert_eq!(static_limit!(LIMIT 20), "LIMIT 20");
        assert_eq!(static_limit!(LIMIT "0"), "LIMIT 0");
        assert_eq!(static_limit!(LIMIT "ALL"), "LIMIT ALL");
        assert_eq!(static_limit!(LIMIT "INVALID"), "LIMIT INVALID");
    }

    #[test]
    fn static_offset_macro() {
        assert_eq!(static_offset!(OFFSET 15), "OFFSET 15");
        assert_eq!(static_offset!(OFFSET 20), "OFFSET 20");
        assert_eq!(static_offset!(OFFSET "0"), "OFFSET 0");
        assert_eq!(static_offset!(OFFSET "ALL"), "OFFSET ALL");
        assert_eq!(static_offset!(OFFSET "INVALID"), "OFFSET INVALID");
    }
}
