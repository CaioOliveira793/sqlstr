use crate::command::{ArgumentBuffer, SqlExpr, WriteSql};

/// **CAST** Command
///
/// A cast specifies how to perform a conversion between two data types.
///
/// # Example
///
/// ```
/// # use squeal_builder::{SqlCommand, Void, SqlExpr, ast::cast};
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

#[macro_export]
macro_rules! static_cast {
    ($expr:literal AS $ty:literal) => {
        concat!("CAST (", $expr, " AS ", $ty, ")")
    };
}

pub use static_cast;

#[cfg(test)]
mod test {
    #[test]
    fn cast() {
        assert_eq!(static_cast!("'-1'" AS "INTEGER"), "CAST ('-1' AS INTEGER)");

        assert_eq!(
            static_cast!("DATE::NOW()" AS "TIMESTAMP"),
            "CAST (DATE::NOW() AS TIMESTAMP)"
        );

        assert_eq!(static_cast!("102" AS "TEXT"), "CAST (102 AS TEXT)");
    }
}
