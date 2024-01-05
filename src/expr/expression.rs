//! Operators and functions.

// TODO: add most of the postgres functions
// https://www.postgresql.org/docs/current/functions.html

pub mod comparison;
pub mod math;

use super::{separator, separator_optional};
use crate::{ArgumentBuffer, SqlExpr, WriteSql};

pub trait BinaryOperator: private::Sealed {
    fn push_operator<Sql, Arg>(&self, sql: &mut Sql)
    where
        Sql: WriteSql<Arg>;
}

pub trait UnaryOperator: private::Sealed {
    fn push_operator<Sql, Arg>(&self, sql: &mut Sql)
    where
        Sql: WriteSql<Arg>;
}

mod private {
    pub trait Sealed {}
}

/// Comparison operators
#[cfg_attr(any(feature = "fmt", test, debug_assertions), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cmp {
    /// Equal `==`
    Eq,
    /// Not equal `!=`
    Neq,
    /// Greater than `>`
    Gt,
    //// Greater than or equal `>=`
    Gte,
    /// Less than `<`
    Lt,
    /// Less than or equal `<=`
    Lte,
}

impl Cmp {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::Neq => "<>",
            Self::Gt => ">",
            Self::Gte => ">=",
            Self::Lt => "<",
            Self::Lte => "<=",
        }
    }
}

impl private::Sealed for Cmp {}

impl BinaryOperator for Cmp {
    fn push_operator<Sql, Arg>(&self, sql: &mut Sql)
    where
        Sql: WriteSql<Arg>,
    {
        sql.push_cmd(self.as_str())
    }
}

/// Logic binary operators
#[cfg_attr(any(feature = "fmt", test, debug_assertions), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogicBi {
    And,
    Or,
}

impl LogicBi {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::And => "AND",
            Self::Or => "OR",
        }
    }
}

impl private::Sealed for LogicBi {}

impl BinaryOperator for LogicBi {
    fn push_operator<Sql, Arg>(&self, sql: &mut Sql)
    where
        Sql: WriteSql<Arg>,
    {
        sql.push_cmd(self.as_str())
    }
}

/// Logic unary operators
#[cfg_attr(any(feature = "fmt", test, debug_assertions), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogicUn {
    Not,
}

impl LogicUn {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            LogicUn::Not => "NOT",
        }
    }
}

impl private::Sealed for LogicUn {}

impl UnaryOperator for LogicUn {
    fn push_operator<Sql, Arg>(&self, sql: &mut Sql)
    where
        Sql: WriteSql<Arg>,
    {
        sql.push_cmd(self.as_str())
    }
}

pub fn continue_condition<Sql, Arg>(sql: &mut Sql, op: LogicBi)
where
    Sql: WriteSql<Arg>,
{
    if sql.as_command().is_empty() {
        return;
    }

    let end = sql.as_command().trim_end_matches(' ');
    // WHERE | ON | HAVING | ( <open group>
    if end.ends_with("WHERE")
        || end.ends_with("ON")
        || end.ends_with("HAVING")
        || end.ends_with('(')
        || sql.as_command().is_empty()
    {
        return;
    }

    separator_optional(sql);
    sql.push_cmd(op.as_str());
}

pub fn lhs_binary_rhs<Sql, Arg, BOp, Lhs, Rhs>(
    sql: &mut Sql,
    lhs: SqlExpr<Lhs>,
    op: BOp,
    rhs: SqlExpr<Rhs>,
) -> Result<(), <Arg as ArgumentBuffer<Lhs>>::Error>
where
    Sql: WriteSql<Arg>,
    BOp: BinaryOperator,
    Arg: ArgumentBuffer<Lhs>,
    Arg: ArgumentBuffer<Rhs, Error = <Arg as ArgumentBuffer<Lhs>>::Error>,
{
    separator_optional(sql);

    sql.push_expr(lhs)?;
    separator(sql);
    op.push_operator(sql);
    separator(sql);
    sql.push_expr(rhs)
}

pub fn binary_rhs<Sql, Arg, BOp, Rhs>(
    sql: &mut Sql,
    op: BOp,
    rhs: SqlExpr<Rhs>,
) -> Result<(), Arg::Error>
where
    Sql: WriteSql<Arg>,
    BOp: BinaryOperator,
    Arg: ArgumentBuffer<Rhs>,
{
    separator_optional(sql);

    op.push_operator(sql);
    separator(sql);
    sql.push_expr(rhs)
}

pub fn unary_rhs<Sql, Arg, UOp, Rhs>(
    sql: &mut Sql,
    op: UOp,
    rhs: SqlExpr<Rhs>,
) -> Result<(), Arg::Error>
where
    Sql: WriteSql<Arg>,
    UOp: UnaryOperator,
    Arg: ArgumentBuffer<Rhs>,
{
    separator_optional(sql);

    op.push_operator(sql);
    separator(sql);
    sql.push_expr(rhs)
}

#[cfg(test)]
mod test {
    use super::{Cmp, LogicBi};
    use crate::{
        expr::{
            binary_rhs, continue_condition, lhs_binary_rhs, math::MathBi, unary_rhs, Group, LogicUn,
        },
        sqlexpr, sqlvalue,
        test::TestArgs,
        SqlCommand, SqlExpr,
    };

    #[test]
    fn condition_comparison() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();
        continue_condition(&mut sql, LogicBi::And);
        {
            let mut group = Group::open(&mut sql);
            lhs_binary_rhs(
                &mut group,
                sqlexpr::<&str>("user.id"),
                Cmp::Eq,
                sqlvalue(32),
            )
            .unwrap();
            continue_condition(&mut group, LogicBi::And);
            lhs_binary_rhs(
                &mut group,
                sqlexpr::<&str>("access.created"),
                Cmp::Gte,
                sqlvalue(2040),
            )
            .unwrap();
        }

        assert_eq!(sql.as_command(), "(user.id = $1 AND access.created >= $2)");
        assert_eq!(sql.arguments.as_str(), "32;2040;");
    }

    #[test]
    fn condition_math() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();
        continue_condition(&mut sql, LogicBi::And);

        {
            let mut group = Group::open(&mut sql);
            lhs_binary_rhs(
                &mut group,
                sqlexpr::<u8>("column1"),
                MathBi::Add,
                SqlExpr::Value(30),
            )
            .unwrap();
            binary_rhs(&mut group, Cmp::Gt, sqlexpr::<u8>("column2")).unwrap();

            continue_condition(&mut group, LogicBi::And);

            lhs_binary_rhs(
                &mut group,
                sqlexpr::<u8>("column1"),
                Cmp::Eq,
                sqlexpr::<u8>("column3"),
            )
            .unwrap();
        }

        continue_condition(&mut sql, LogicBi::Or);

        unary_rhs(&mut sql, LogicUn::Not, sqlexpr::<u8>("column4")).unwrap();
        binary_rhs(&mut sql, Cmp::Lt, SqlExpr::Value(10)).unwrap();

        assert_eq!(
            sql.as_command(),
            "(column1 + $1 > column2 AND column1 = column3) OR NOT column4 < $2"
        );
    }
}
