use super::separator_optional;
use crate::WriteSql;

/// Write a `INSERT INTO` clause to start a insert sql command.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::{insert};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// insert(&mut sql);
///
/// assert_eq!(sql.as_command(), "INSERT INTO");
/// # Ok(())
/// # }
/// ```
pub fn insert<Sql, Arg>(sql: &mut Sql)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("INSERT INTO");
}

/// Write an `INSERT INTO <table>` clause to start an insert command with a
/// table.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::{insert_into};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// insert_into(&mut sql, "user");
///
/// assert_eq!(sql.as_command(), "INSERT INTO user");
/// # Ok(())
/// # }
/// ```
pub fn insert_into<Sql, Arg>(sql: &mut Sql, table: &str)
where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("INSERT INTO ");
    sql.push_cmd(table);
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConflictTarget<'expr> {
    Constraint(&'expr str),
    IndexColumn(&'expr str),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConflictAction {
    Nothing,
    Update,
}

/// Writes a `ON CONFLICT <target> DO <action>` clause to specify an
/// alternative action during conflict in a `INSERT` clause.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::{on_conflict, ConflictAction, ConflictTarget};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// on_conflict(&mut sql, Some(ConflictTarget::Constraint("pkey_user")), ConflictAction::Update);
///
/// assert_eq!(sql.as_command(), "ON CONFLICT ON CONSTRAINT pkey_user DO UPDATE");
/// # Ok(())
/// # }
/// ```
pub fn on_conflict<'expr, Sql, Arg>(
    sql: &mut Sql,
    target: Option<ConflictTarget<'expr>>,
    action: ConflictAction,
) where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("ON CONFLICT");

    if let Some(target) = target {
        match target {
            ConflictTarget::Constraint(constr_name) => {
                sql.push_cmd(" ON CONSTRAINT ");
                sql.push_cmd(constr_name);
            }
            ConflictTarget::IndexColumn(indx_expr) => {
                sql.push_cmd(" ");
                sql.push_cmd(indx_expr);
            }
        }
    }

    match action {
        ConflictAction::Nothing => {
            sql.push_cmd(" DO NOTHING");
        }
        ConflictAction::Update => {
            sql.push_cmd(" DO UPDATE");
        }
    }
}
