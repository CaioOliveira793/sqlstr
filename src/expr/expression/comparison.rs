use crate::{
    expr::{separator_optional, Group},
    ArgumentBuffer, SqlExpr, WriteSql,
};

pub fn between<Sql, Arg, Lhs, Rhs>(
    sql: &mut Sql,
    lhs: SqlExpr<Lhs>,
    rhs: SqlExpr<Rhs>,
) -> Result<(), <Arg as ArgumentBuffer<Lhs>>::Error>
where
    Sql: WriteSql<Arg>,
    Arg: ArgumentBuffer<Lhs>,
    Arg: ArgumentBuffer<Rhs, Error = <Arg as ArgumentBuffer<Lhs>>::Error>,
{
    separator_optional(sql);

    sql.push_cmd("BETWEEN ");
    sql.push_expr(lhs)?;
    sql.push_cmd(" AND ");
    sql.push_expr(rhs)
}

pub fn is_null<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);

    sql.push_cmd("IS NULL");
}

pub fn is_not_null<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);

    sql.push_cmd("IS NOT NULL");
}

pub fn is_in<Sql, Arg>(sql: &mut Sql) -> Group<'_, Sql, Arg>
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);

    sql.push_cmd("IN ");
    Group::open(sql)
}
