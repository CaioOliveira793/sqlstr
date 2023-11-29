use alloc::string::String;

use super::{FromTable, Tables};
use crate::{error::SqlError, macros::map_intermediate_sql};

pub struct SelectColumn<Arg> {
    pub(super) command: String,
    pub(super) arguments: Arg,
}

impl<Arg> SelectColumn<Arg> {
    /// Add a column into the SELECT command
    ///
    /// # Example
    ///
    /// ```
    /// # use squeal_builder::{select::*, Void, error::SqlError};
    /// # use core::convert::Infallible;
    /// # fn main() -> Result<(), SqlError<Infallible>> {
    /// let cmd = select(Void::new())
    ///     .column("first_name")?
    ///     .column("last_name")?;
    ///
    /// assert_eq!(cmd.as_str(), "SELECT first_name, last_name");
    /// # Ok(())
    /// # }
    /// ```
    pub fn column<EArg>(mut self, column: &str) -> Result<Self, SqlError<EArg>> {
        self.command.push(',');
        self.transition_column(column)
    }

    pub(super) fn transition_column<EArg>(mut self, column: &str) -> Result<Self, SqlError<EArg>> {
        self.command.try_reserve(column.len() + 1)?;
        self.command.push(' ');
        self.command.push_str(column);
        Ok(self)
    }

    /// Add a column with a alias into the SELECT command
    ///
    /// # Example
    ///
    /// ```
    /// # use squeal_builder::{select::*, Void, error::SqlError};
    /// # use core::convert::Infallible;
    /// # fn main() -> Result<(), SqlError<Infallible>> {
    /// let cmd = select(Void::new())
    ///     .column_as("firstName", "first_name")?
    ///     .column_as("lastName", "last_name")?;
    ///
    /// assert_eq!(cmd.as_str(), "SELECT firstName AS first_name, lastName AS last_name");
    /// # Ok(())
    /// # }
    /// ```
    pub fn column_as<EArg>(mut self, column: &str, alias: &str) -> Result<Self, SqlError<EArg>> {
        self.command.push(',');
        self.transition_column_as(column, alias)
    }

    pub(super) fn transition_column_as<EArg>(
        mut self,
        column: &str,
        alias: &str,
    ) -> Result<Self, SqlError<EArg>> {
        self.command.try_reserve(column.len() + alias.len() + 5)?;
        self.command.push(' ');
        self.command.push_str(column);
        self.command.push_str(" AS ");
        self.command.push_str(alias);
        Ok(self)
    }

    pub fn from<EArg>(self, table: &str) -> Result<FromTable<Arg>, SqlError<EArg>> {
        let mut sql = map_intermediate_sql!(FromTable, self);
        sql.command.push_str(" FROM");
        sql.transition_from(table)
    }

    pub fn static_from<EArg>(self, tables: Tables) -> Result<FromTable<Arg>, SqlError<EArg>> {
        let mut sql = map_intermediate_sql!(FromTable, self);
        sql.command.push_str(" FROM");
        sql.transition_static_from(tables)
    }

    pub fn as_str(&self) -> &str {
        &self.command
    }
}
