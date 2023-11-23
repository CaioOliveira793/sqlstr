use alloc::string::String;

use super::Tables;
use crate::{
    error::SqlError,
    macros::{display_sql_command, map_intermediate_sql},
    SqlCommand,
};

/// Starts a `FROM` section to push table names
pub struct FromTable<Arg> {
    pub(super) command: String,
    pub(super) arguments: Arg,
}

impl<Arg> FromTable<Arg> {
    pub fn from<EArg>(mut self, table: &str) -> Result<Self, SqlError<EArg>> {
        self.command.push(',');
        self.transition_from(table)
    }

    pub(super) fn transition_from<EArg>(
        mut self,
        table: &str,
    ) -> Result<FromTable<Arg>, SqlError<EArg>> {
        self.command.try_reserve(table.len() + 1)?;
        self.command.push(' ');
        self.command.push_str(table);
        Ok(self)
    }

    pub fn static_from<EArg>(mut self, tables: Tables) -> Result<FromTable<Arg>, SqlError<EArg>> {
        self.command.push(',');
        self.transition_static_from(tables)
    }

    pub(super) fn transition_static_from<EArg>(
        mut self,
        tables: Tables,
    ) -> Result<FromTable<Arg>, SqlError<EArg>> {
        self.command.try_reserve(tables.0.len() + 1)?;
        self.command.push(' ');
        self.command.push_str(tables.0);
        Ok(self)
    }

    pub fn end(self) -> SqlCommand<Arg> {
        map_intermediate_sql!(SqlCommand, self)
    }
}

display_sql_command!(FromTable);
