use alloc::{borrow::Cow, string::String, vec::Vec};
use core::{borrow::Borrow, ops::Deref};

#[cfg_attr(any(feature = "fmt", test, debug_assertions), derive(Debug))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColumnExpr<'c>(Cow<'c, str>);

impl<'c> ColumnExpr<'c> {
    pub const fn new(column_expr: Cow<'c, str>) -> Self {
        Self(column_expr)
    }

    pub fn from_list(list: ColumnExprList<'c>) -> Self {
        let list = list.0.join(", ");
        Self(Cow::Owned(list))
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

    pub fn column(&mut self, column: Cow<'c, str>) {
        self.0.push(column);
    }

    pub fn column_as(&mut self, column: Cow<'c, str>, alias: Cow<'c, str>) {
        let mut col = String::with_capacity(column.len() + alias.len() + 4);
        col.push_str(&column);
        col.push_str(" AS ");
        col.push_str(&alias);
        self.0.push(Cow::Owned(col));
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

    pub fn end(self) -> ColumnExpr<'c> {
        ColumnExpr::from_list(self)
    }
}

impl<'c> From<ColumnExprList<'c>> for ColumnExpr<'c> {
    fn from(value: ColumnExprList<'c>) -> Self {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn columns_list() {
        let mut columns = column_list();
        columns.column("id".into());
        columns.column_as("firstName".into(), "first_name".into());
        columns.column("last_name".into());
        let columns = columns.end();

        assert_eq!(columns.as_str(), "id, firstName AS first_name, last_name");
    }

    #[test]
    fn columns_list_aliased() {
        let mut columns = column_list();
        columns.column_as("identifier".into(), "id".into());
        columns.column_as("u.fullName".into(), "full_name".into());
        let columns = columns.end();

        assert_eq!(
            columns.as_str(),
            "identifier AS id, u.fullName AS full_name"
        );
    }

    #[test]
    fn columns_extended() {
        let mut columns = column_list();
        columns.column_as("identifier".into(), "id".into());
        columns.column_as("u.fullName".into(), "full_name".into());

        let mut cols = column_list();
        cols.column("pass".into());
        cols.column_as("org".into(), "organization".into());

        columns.extend(cols.as_slice());
        let columns = columns.end();

        assert_eq!(
            columns.as_str(),
            "identifier AS id, u.fullName AS full_name, pass, org AS organization"
        );
    }
}
