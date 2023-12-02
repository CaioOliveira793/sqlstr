//! Postgres supported functions and operators
//!
//! https://www.postgresql.org/docs/current/functions.html

use super::separator_optional;
use crate::{ArgumentBuffer, SqlExpr, WriteSql};

/// Postgres comparison operators
///
/// https://www.postgresql.org/docs/current/functions-comparison.html
#[cfg_attr(any(feature = "fmt", test, debug_assertions), derive(Debug))]
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

pub fn push_comparison<Sql, Arg, T>(
    sql: &mut Sql,
    op: ComparisonOp,
    rhs: SqlExpr<'_, T>,
) -> Result<(), Arg::Error>
where
    Sql: WriteSql<Arg>,
    Arg: ArgumentBuffer<T>,
{
    separator_optional(sql);
    sql.push_cmd(op.as_str());
    sql.push_cmd(" ");
    sql.push_expr(rhs)
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
    separator_optional(sql);
    sql.push_cmd(op.as_str());
    sql.push_cmd(" ");
    sql.push_expr(rhs)
}

#[allow(unused_macros)]
macro_rules! static_comparison {
    (=) => {
        "="
    };
    (!=) => {
        "<>"
    };
    (>) => {
        ">"
    };
    (<) => {
        "<"
    };
    (>=) => {
        ">="
    };
    (<=) => {
        "<="
    };
}

#[allow(unused_macros)]
macro_rules! static_logical_op {
    (NOT) => {
        "NOT"
    };
    (AND) => {
        "AND"
    };
    (OR) => {
        "OR"
    };
    (AND_NOT) => {
        "AND NOT"
    };
    (OR_NOT) => {
        "OR NOT"
    };
}

#[allow(unused_imports)]
pub(super) use static_comparison;

#[allow(unused_imports)]
pub(super) use static_logical_op;

#[macro_export]
macro_rules! static_condition {
    ($a:literal $op:tt $b:literal) => {
        concat!(
            $a,
            " ",
            $crate::expr::static_comparison!($op),
            " ",
            $b
        )
    };
    ($pre_logic:tt $a:literal $op:tt $b:literal) => {
        concat!(
            $crate::expr::static_logical_op!($pre_logic),
            " ",
            $a,
            " ",
            $crate::expr::static_comparison!($op),
            " ",
            $b
        )
    };

    ($a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)+) => {
        concat!(
            $crate::expr::static_condition!($a $op $b),
            $(
                " ",
                $crate::expr::static_logical_op!($logic_op),
                " ",
                $crate::expr::static_condition!($ax $opx $bx)
            )+
        )
    };
    ($pre_logic:tt $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)+) => {
        concat!(
            $crate::expr::static_logical_op!($pre_logic),
            " ",
            $crate::expr::static_condition!($a $op $b),
            $(
                " ",
                $crate::expr::static_logical_op!($logic_op),
                " ",
                $crate::expr::static_condition!($ax $opx $bx)
            )+
        )
    };
}

pub use static_condition;

#[cfg(test)]
mod test {
    use super::{ComparisonOp, LogicalOp};
    use crate::{
        expr::{push_comparison, push_logic, Group},
        sqlexpr, sqlvalue,
        test::TestArgs,
        SqlCommand, WriteSql,
    };

    #[test]
    fn static_condition_macro() {
        assert_eq!(
            static_condition!(NOT "access_history.user_id" = "user.id"),
            "NOT access_history.user_id = user.id"
        );
        assert_eq!(
            static_condition!("access_history.user_id" > "user.id"),
            "access_history.user_id > user.id"
        );
        assert_eq!(
            static_condition!(AND_NOT "access_history.user_id" >= "user.id"),
            "AND NOT access_history.user_id >= user.id"
        );
        assert_eq!(
            static_condition!("user.id" = "access_history.user_id" OR "user.updated" < "access_history.created"),
            "user.id = access_history.user_id OR user.updated < access_history.created"
        );
        assert_eq!(
            static_condition!("user.id" != "access_history.user_id"),
            "user.id <> access_history.user_id"
        );
        assert_eq!(
            static_condition!(NOT "user.id" = "access_history.user_id" AND_NOT "user.updated" < "access_history.created"),
            "NOT user.id = access_history.user_id AND NOT user.updated < access_history.created"
        )
    }

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
