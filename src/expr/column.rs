use alloc::{borrow::Cow, string::String, vec::Vec};
use core::{borrow::Borrow, ops::Deref};

use super::separator_optional;
use crate::WriteSql;

/// Write a list of columns from a iterator into the sql command buffer.
///
/// # Example
///
/// ```
/// # use squeal::{SqlCommand, Void, SqlExpr};
/// # use squeal::expr::{columns_iter};
/// # use core::convert::Infallible;
/// # fn main() -> Result<(), Infallible> {
/// let mut sql: SqlCommand<Void> = SqlCommand::default();
/// columns_iter(&mut sql, ["id", "name", "email", "created"]);
///
/// assert_eq!(sql.as_command(), "id, name, email, created");
/// # Ok(())
/// # }
/// ```
pub fn columns_iter<'c, Sql, Arg, I>(sql: &mut Sql, columns: I)
where
    Sql: WriteSql<Arg>,
    I: IntoIterator<Item = &'c str>,
{
    separator_optional(sql);

    let mut cols = columns.into_iter();
    if let Some(first) = cols.next() {
        sql.push_cmd(first);
    }

    for col in cols {
        sql.push_cmd(", ");
        sql.push_cmd(col);
    }
}

#[cfg_attr(any(feature = "fmt", test, debug_assertions), derive(Debug))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnExpr<'c>(Cow<'c, str>);

impl<'c> ColumnExpr<'c> {
    pub const fn new(column_expr: Cow<'c, str>) -> Self {
        Self(column_expr)
    }

    pub fn from_list(list: &ColumnExprList<'c>) -> Self {
        Self(Cow::Owned(list.0.join(", ")))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'c> AsRef<str> for ColumnExpr<'c> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'c> Borrow<str> for ColumnExpr<'c> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl<'c> Deref for ColumnExpr<'c> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg_attr(any(feature = "fmt", test, debug_assertions), derive(Debug))]
pub struct ColumnExprList<'c>(Vec<Cow<'c, str>>);

pub const fn column_list<'c>() -> ColumnExprList<'c> {
    ColumnExprList::new()
}

impl<'c> ColumnExprList<'c> {
    pub const ALIAS: &str = "AS";

    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn column<C>(mut self, column: C) -> Self
    where
        C: Into<Cow<'c, str>>,
    {
        self.0.push(column.into());
        self
    }

    pub fn column_as(mut self, column: &str, alias: &str) -> Self {
        let mut col = String::with_capacity(column.len() + alias.len() + 4);
        col.push_str(column);
        col.push_str(" AS ");
        col.push_str(alias);
        self.0.push(Cow::Owned(col));
        self
    }

    pub fn as_slice(&self) -> &[Cow<'c, str>] {
        &self.0
    }

    pub fn extend<'l>(&mut self, other: &[Cow<'l, str>])
    where
        'l: 'c,
    {
        self.0.extend_from_slice(other);
    }

    pub fn expr(&self) -> ColumnExpr<'c> {
        ColumnExpr::from_list(self)
    }
}

impl<'c> From<&ColumnExprList<'c>> for ColumnExpr<'c> {
    fn from(value: &ColumnExprList<'c>) -> Self {
        Self::from_list(value)
    }
}

impl<'c> Borrow<[Cow<'c, str>]> for ColumnExprList<'c> {
    fn borrow(&self) -> &[Cow<'c, str>] {
        &self.0
    }
}

impl<'c> Deref for ColumnExprList<'c> {
    type Target = [Cow<'c, str>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[macro_export]
macro_rules! static_columns {
    ($column:literal) => {
        $column
    };

    ($column:literal AS $alias:literal) => {
        concat!($column, " AS ", $alias)
    };

    ($fcolumn:literal $(AS $falias:literal)?, $($column:literal $(AS $alias:literal)?),* $(,)?) => {
        concat!($fcolumn $(, " AS ", $falias)?, $(", ", $column $(, " AS ", $alias)?),*)
    };
}

pub use static_columns;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        expr::{from_tables, select, separator_optional},
        test::{TestArgs, User},
        SqlCommand,
    };

    #[test]
    fn static_columns_test() {
        assert_eq!(static_columns!("id"), "id");
        assert_eq!(static_columns!("'quoted column'",), "'quoted column'");
        assert_eq!(static_columns!("id", "created"), "id, created");
        assert_eq!(
            static_columns!("id", "name", "deleted",),
            "id, name, deleted"
        );
        assert_eq!(static_columns!("userName" AS "name",), "userName AS name");
        assert_eq!(
            static_columns!("userName" AS "name", "id",),
            "userName AS name, id"
        );
        assert_eq!(
            static_columns!("userName" AS "name", "ID" AS "id"),
            "userName AS name, ID AS id"
        );
        assert_eq!(
            static_columns!("userName" AS "name", "age", "ID" AS "id"),
            "userName AS name, age, ID AS id"
        );
        assert_eq!(
            static_columns!("name", "age", "ID" AS "id"),
            "name, age, ID AS id"
        );
        assert_eq!(
            static_columns!("n" AS "name", "a" AS "age", "ID" AS "id"),
            "n AS name, a AS age, ID AS id"
        );
    }

    #[test]
    fn columns_list() {
        let columns = column_list()
            .column("id")
            .column_as("firstName", "first_name")
            .column("last_name")
            .expr();

        assert_eq!(columns.as_str(), "id, firstName AS first_name, last_name");
    }

    #[test]
    fn columns_list_aliased() {
        let columns = column_list()
            .column_as("identifier", "id")
            .column_as("u.fullName", "full_name")
            .expr();

        assert_eq!(
            columns.as_str(),
            "identifier AS id, u.fullName AS full_name"
        );
    }

    #[test]
    fn columns_extended() {
        let mut columns = column_list()
            .column_as("identifier", "id")
            .column_as("u.fullName", "full_name");

        let cols = column_list()
            .column("pass")
            .column_as("org", "organization");

        columns.extend(cols.as_slice());
        let columns = columns.expr();

        assert_eq!(
            columns.as_str(),
            "identifier AS id, u.fullName AS full_name, pass, org AS organization"
        );
    }

    #[test]
    fn select_columns_from_table() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        select(&mut sql);
        let columns = column_list()
            .column(User::Id.as_str())
            .column_as("created_at", User::Created.as_str())
            .column(User::Name.as_str())
            .expr();
        separator_optional(&mut sql);
        sql.push_cmd(columns.as_str());
        from_tables(&mut sql, [User::TABLE]);

        assert_eq!(
            sql.as_command(),
            "SELECT id, created_at AS created, name FROM user"
        );
        assert_eq!(sql.arguments.as_str(), "");
    }
}
