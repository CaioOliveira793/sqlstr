//! Postgres supported functions and operators
//!
//! https://www.postgresql.org/docs/current/functions.html

use crate::command::SqlExpr;
use crate::{ArgumentBuffer, WriteSql};

/// Postgres comparison operators
///
/// https://www.postgresql.org/docs/current/functions-comparison.html
#[cfg_attr(any(feature = "fmt", test), derive(Debug))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Is,
}

impl ComparisonOp {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Equal => "=",
            Self::NotEqual => "<>",
            Self::GreaterThan => ">",
            Self::GreaterThanOrEqual => ">=",
            Self::LessThan => "<",
            Self::LessThanOrEqual => "<=",
            Self::Is => "IS",
        }
    }
}

/// Postgres logical operators
///
/// https://www.postgresql.org/docs/current/functions-logical.html
pub enum LogicalOp {
    And,
    Or,
    Not,
}

impl LogicalOp {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::And => "AND",
            Self::Or => "OR",
            Self::Not => "NOT",
        }
    }
}

/// Postgres available Mathematical functions and operators
///
/// - [Operators](https://www.postgresql.org/docs/current/functions-math.html)
pub enum MathOp {
    /// Adition
    ///
    /// 2 + 3
    Addition,
    /// Subtration
    ///
    /// 2 - 3
    Subtraction,
    /// Negation
    ///
    /// - (-1)
    Negation,
    /// Multiplication
    ///
    /// 5 * 2
    Multiplication,
    Division,
    Modulo,
    Exponentiation,
    SquareRoot,
    CubeRoot,
    /// Factorial prefix operator
    ///
    /// !! 5
    Factorial,
    Absolute,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    ShiftLeft,
    ShiftRight,
}

// pub struct ConditionExpr<'le, 're, Lhs, Rhs> {
//     lhs: Operand<'le, Lhs>,
//     op: Operator,
//     rhs: Operand<'re, Rhs>,
// }

// pub struct ConditionExpr<'cmd, Sql: WriteSql<Arg>, Arg>(&'cmd mut Sql, PhantomData<Arg>);

pub fn push_comparison<Sql, Arg, T>(
    sql: &mut Sql,
    op: ComparisonOp,
    rhs: SqlExpr<'_, T>,
) -> Result<(), Arg::Error>
where
    Sql: WriteSql<Arg>,
    Arg: ArgumentBuffer<T>,
{
    sql.push_cmd(" ");
    sql.push_cmd(op.as_str());
    sql.push_cmd(" ");
    sql.push_expr(rhs)?;
    Ok(())
}

pub fn push_logic<Sql, Arg, T>(
    sql: &mut Sql,
    op: LogicalOp,
    rhs: SqlExpr<'_, T>,
) -> Result<(), Arg::Error>
where
    Sql: WriteSql<Arg>,
    Arg: ArgumentBuffer<T>,
{
    sql.push_cmd(" ");
    sql.push_cmd(op.as_str());
    sql.push_cmd(" ");
    sql.push_expr(rhs)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{ComparisonOp, LogicalOp};
    use crate::{
        ast::{push_comparison, push_logic, Group},
        sqlexpr, sqlvalue,
        test::TestArgs,
        SqlCommand, WriteSql,
    };

    #[test]
    fn condition_expr() {
        let mut sql: SqlCommand<TestArgs> = Default::default();
        {
            let mut group = Group::open(&mut sql);
            group.push_cmd("user.id");
            push_comparison(&mut group, ComparisonOp::Equal, sqlvalue(32)).unwrap();
            push_logic(
                &mut group,
                LogicalOp::And,
                sqlexpr::<&str>("access.created"),
            )
            .unwrap();
            push_comparison(&mut group, ComparisonOp::GreaterThanOrEqual, sqlvalue(2040)).unwrap();
        }

        assert_eq!(sql.as_command(), "(user.id = $1 AND access.created >= $2)");
        assert_eq!(sql.arguments.as_str(), "32;2040;");
    }
}
