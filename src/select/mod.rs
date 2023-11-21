use alloc::string::String;

use crate::error::SqlError;
use crate::format_num::format_u32_base10;
use crate::macros::{display_sql_command, map_intermediate_sql};
use crate::{ArgumentBuffer, SqlCommand};

pub use macros::*;

mod macros;

pub fn select<Arg>(arguments: Arg) -> Select<Arg> {
    Select::new(arguments)
}

pub struct Select<Arg> {
    command: String,
    arguments: Arg,
}

impl<Arg> Select<Arg> {
    pub fn new(arguments: Arg) -> Self {
        Self {
            arguments,
            command: String::from("SELECT"),
        }
    }

    pub fn column<EArg>(mut self, column: &str) -> Result<PushColumn<Arg>, SqlError<EArg>> {
        self.command.try_reserve(column.len() + 1)?;
        self.command.push(' ');
        self.command.push_str(column);
        Ok(map_intermediate_sql!(PushColumn, self))
    }

    pub fn columns<EArg>(mut self, columns: &[&str]) -> Result<PushColumn<Arg>, SqlError<EArg>> {
        // each column + ", " - 1 (for the first, which only use a ' ')
        let total_length = columns.iter().map(|s| s.len() + 2).sum::<usize>() - 1;
        self.command.try_reserve(total_length)?;

        let first = columns.first().ok_or(SqlError::ArgumentNotFound)?;
        self.command.push(' ');
        self.command.push_str(first);

        for column in &columns[1..] {
            self.command.push_str(", ");
            self.command.push_str(column);
        }

        Ok(map_intermediate_sql!(PushColumn, self))
    }

    pub fn static_columns<EArg>(
        mut self,
        columns: Columns,
    ) -> Result<FromTable<Arg>, SqlError<EArg>> {
        self.command.try_reserve(columns.0.len())?;

        self.command.push(' ');
        self.command.push_str(columns.0);

        Ok(map_intermediate_sql!(FromTable, self))
    }

    pub fn value<T>(
        mut self,
        value: T,
    ) -> Result<PushValue<Arg>, SqlError<<Arg as ArgumentBuffer<T>>::Error>>
    where
        Arg: ArgumentBuffer<T>,
    {
        self.arguments.push(value).map_err(SqlError::Argument)?;

        let mut buf = [0; 10];
        self.command.push_str(" $");
        self.command
            .push_str(format_u32_base10(self.arguments.count(), &mut buf));

        Ok(map_intermediate_sql!(PushValue, self))
    }

    pub fn values<T, I>(
        mut self,
        values: I,
    ) -> Result<SqlCommand<Arg>, SqlError<<Arg as ArgumentBuffer<T>>::Error>>
    where
        Arg: ArgumentBuffer<T>,
        I: IntoIterator<Item = T>,
    {
        let mut values = values.into_iter();
        let first = values.next().ok_or(SqlError::ArgumentNotFound)?;
        self.arguments.push(first).map_err(SqlError::Argument)?;

        let mut buf = [0; 10];
        self.command.push_str(" $");
        self.command
            .push_str(format_u32_base10(self.arguments.count(), &mut buf));

        for value in values {
            self.arguments.push(value).map_err(SqlError::Argument)?;

            self.command.push_str(", $");
            self.command
                .push_str(format_u32_base10(self.arguments.count(), &mut buf));
        }

        Ok(map_intermediate_sql!(SqlCommand, self))
    }
}

display_sql_command!(Select);

pub struct PushValue<Arg> {
    command: String,
    arguments: Arg,
}

impl<Arg> PushValue<Arg> {
    pub fn value<T>(mut self, value: T) -> Result<Self, SqlError<<Arg as ArgumentBuffer<T>>::Error>>
    where
        Arg: ArgumentBuffer<T>,
    {
        self.arguments.push(value).map_err(SqlError::Argument)?;

        let mut buf = [0; 10];
        self.command.push_str(", $");
        self.command
            .push_str(format_u32_base10(self.arguments.count(), &mut buf));

        Ok(self)
    }

    pub fn end(self) -> SqlCommand<Arg> {
        map_intermediate_sql!(SqlCommand, self)
    }
}

display_sql_command!(PushValue);

pub struct PushColumn<Arg> {
    command: String,
    arguments: Arg,
}

impl<Arg> PushColumn<Arg> {
    pub fn column<EArg>(mut self, column: &str) -> Result<Self, SqlError<EArg>> {
        self.command.try_reserve(column.len() + 2)?;
        self.command.push_str(", ");
        self.command.push_str(column);
        Ok(self)
    }

    pub fn from_table<EArg>(self, table: &str) -> Result<PushFromTable<Arg>, SqlError<EArg>> {
        let sql = map_intermediate_sql!(FromTable, self);
        sql.from_table(table)
    }

    pub fn static_from_tables<EArg>(
        self,
        tables: Tables,
    ) -> Result<PushFromTable<Arg>, SqlError<EArg>> {
        let from_table = map_intermediate_sql!(FromTable, self);
        from_table.static_from_tables(tables)
    }
}

display_sql_command!(PushColumn);

pub struct FromTable<Arg> {
    command: String,
    arguments: Arg,
}

/// Starts a `FROM` section to push table names
impl<Arg> FromTable<Arg> {
    pub fn from_table<EArg>(mut self, table: &str) -> Result<PushFromTable<Arg>, SqlError<EArg>> {
        self.command.try_reserve(table.len() + 6)?;
        self.command.push_str(" FROM ");
        self.command.push_str(table);
        Ok(map_intermediate_sql!(PushFromTable, self))
    }

    pub fn static_from_tables<EArg>(
        mut self,
        tables: Tables,
    ) -> Result<PushFromTable<Arg>, SqlError<EArg>> {
        self.command.try_reserve(tables.0.len() + 6)?;
        self.command.push_str(" FROM ");
        self.command.push_str(tables.0);
        Ok(map_intermediate_sql!(PushFromTable, self))
    }

    pub fn end(self) -> SqlCommand<Arg> {
        map_intermediate_sql!(SqlCommand, self)
    }
}

display_sql_command!(FromTable);

/// Push table names in a `FROM` section
#[derive(Debug)]
pub struct PushFromTable<Arg> {
    command: String,
    arguments: Arg,
}

impl<Arg> PushFromTable<Arg> {
    pub fn from<EArg>(mut self, table: &str) -> Result<Self, SqlError<EArg>> {
        self.command.try_reserve(table.len() + 2)?;
        self.command.push_str(", ");
        self.command.push_str(table);
        Ok(self)
    }

    pub fn where_clause(self) -> PushWhereClause<Arg> {
        map_intermediate_sql!(PushWhereClause, self)
    }

    pub fn end(self) -> SqlCommand<Arg> {
        map_intermediate_sql!(SqlCommand, self)
    }
}

display_sql_command!(PushFromTable);

pub struct PushWhereClause<Arg> {
    command: String,
    arguments: Arg,
}

impl<Arg> PushWhereClause<Arg> {
    pub fn end(self) -> SqlCommand<Arg> {
        map_intermediate_sql!(SqlCommand, self)
    }
}

display_sql_command!(PushWhereClause);

#[cfg(test)]
mod test;
