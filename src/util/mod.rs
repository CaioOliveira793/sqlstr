mod macros;

pub use macros::*;

use crate::{command::SqlExpr, ArgumentBuffer, WriteSql};

pub fn item_sep<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    sql.push_cmd(", ");
}

pub fn item_sep_optional<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    if !sql.as_command().ends_with(", ") {
        sql.push_cmd(", ");
    }
}

pub fn sep_optional<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    if !sql.as_command().ends_with(' ') {
        sql.push_cmd(" ");
    }
}

pub fn sep<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    sql.push_cmd(" ");
}

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
