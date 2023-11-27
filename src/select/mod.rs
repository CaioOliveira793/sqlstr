use alloc::string::String;

use crate::error::SqlError;
use crate::macros::{display_sql_command, map_intermediate_sql};
use crate::ArgumentBuffer;

pub use columns::*;
pub use from::*;
pub use join::*;
pub use macros::*;
pub use values::*;

mod columns;
mod from;
mod join;
mod macros;
mod values;

pub fn select<Arg>(arguments: Arg) -> Select<Arg> {
    Select::new(arguments)
}

pub fn select_all<Arg>(arguments: Arg) -> Select<Arg> {
    Select::all(arguments)
}

pub fn select_distinct<Arg>(arguments: Arg) -> Select<Arg> {
    Select::distinct(arguments)
}

pub struct Select<Arg> {
    command: String,
    arguments: Arg,
}

impl<Arg> Select<Arg> {
    /// SELECT
    ///
    /// The select command retrieves rows from zero or more tables.
    fn new(arguments: Arg) -> Self {
        Self {
            arguments,
            command: String::from("SELECT"),
        }
    }

    /// SELECT ALL
    ///
    /// The select query will return all the candidate rows, including duplicates.
    ///
    /// Generally the database default
    fn all(arguments: Arg) -> Self {
        Self {
            arguments,
            command: String::from("SELECT ALL"),
        }
    }

    /// SELECT DISTINCT|DISTINCT ROW
    ///
    /// The select query will return only the distinct cantidate rows, eliminating duplicates.
    fn distinct(arguments: Arg) -> Self {
        Self {
            arguments,
            command: String::from("SELECT DISTINCT"),
        }
    }

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
    pub fn column<EArg>(self, column: &str) -> Result<SelectColumn<Arg>, SqlError<EArg>> {
        let sql = map_intermediate_sql!(SelectColumn, self);
        sql.transition_column(column)
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
    pub fn column_as<EArg>(
        self,
        column: &str,
        alias: &str,
    ) -> Result<SelectColumn<Arg>, SqlError<EArg>> {
        let sql = map_intermediate_sql!(SelectColumn, self);
        sql.transition_column_as(column, alias)
    }

    pub fn static_columns<EArg>(
        self,
        columns: Columns,
    ) -> Result<SelectColumn<Arg>, SqlError<EArg>> {
        let sql = map_intermediate_sql!(SelectColumn, self);
        sql.transition_static_columns(columns)
    }

    pub fn value<T>(
        self,
        value: T,
    ) -> Result<SelectValue<Arg>, SqlError<<Arg as ArgumentBuffer<T>>::Error>>
    where
        Arg: ArgumentBuffer<T>,
    {
        let sql = map_intermediate_sql!(SelectValue, self);
        sql.transition_value(value)
    }

    pub fn values<T, I>(
        self,
        values: I,
    ) -> Result<SelectValue<Arg>, SqlError<<Arg as ArgumentBuffer<T>>::Error>>
    where
        Arg: ArgumentBuffer<T>,
        I: IntoIterator<Item = T>,
    {
        let sql = map_intermediate_sql!(SelectValue, self);
        sql.transition_values(values)
    }
}

display_sql_command!(Select);

#[cfg(test)]
mod test;
