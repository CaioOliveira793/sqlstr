use alloc::string::String;

use crate::{error::SqlError, macros::map_intermediate_sql};

pub struct Join<Arg> {
    pub(super) command: String,
    pub(super) arguments: Arg,
}

impl<Arg> Join<Arg> {
    pub fn cross_join<EArg>(mut self, table: &str) -> Result<Self, SqlError<EArg>> {
        self.command.try_reserve(table.len() + 12)?;
        self.command.push_str(" CROSS JOIN ");
        self.command.push_str(table);
        Ok(self)
    }

    pub fn inner_join<EArg>(mut self, table: &str) -> Result<JoinCondition<Arg>, SqlError<EArg>> {
        self.command.push_str(" INNER JOIN ");
        self.command.push_str(table);
        Ok(map_intermediate_sql!(JoinCondition, self))
    }

    pub fn left_join<EArg>(mut self, table: &str) -> Result<JoinCondition<Arg>, SqlError<EArg>> {
        self.command.push_str(" LEFT JOIN ");
        self.command.push_str(table);
        Ok(map_intermediate_sql!(JoinCondition, self))
    }

    pub fn right_join<EArg>(mut self, table: &str) -> Result<JoinCondition<Arg>, SqlError<EArg>> {
        self.command.push_str(" RIGHT JOIN ");
        self.command.push_str(table);
        Ok(map_intermediate_sql!(JoinCondition, self))
    }

    pub fn full_join<EArg>(mut self, table: &str) -> Result<JoinCondition<Arg>, SqlError<EArg>> {
        self.command.push_str(" FULL JOIN ");
        self.command.push_str(table);
        Ok(map_intermediate_sql!(JoinCondition, self))
    }
}

pub struct JoinCondition<Arg> {
    pub(super) command: String,
    pub(super) arguments: Arg,
}

impl<Arg> JoinCondition<Arg> {
    pub fn using<EArg>(mut self, column: &str) -> Result<UsingColumn<Arg>, SqlError<EArg>> {
        self.command.push_str(" USING (");
        self.command.push_str(column);
        Ok(map_intermediate_sql!(UsingColumn, self))
    }

    pub fn on<EArg>(mut self) -> Result<Self, SqlError<EArg>> {
        self.command.push_str(" ON");
        // TODO: boolean_expressions
        Ok(self)
    }
}

pub struct UsingColumn<Arg> {
    pub(super) command: String,
    pub(super) arguments: Arg,
}

impl<Arg> UsingColumn<Arg> {
    pub fn with(mut self, column: &str) -> UsingColumn<Arg> {
        self.command.push_str(", ");
        self.command.push_str(column);
        self
    }

    pub fn end_using(mut self) -> Join<Arg> {
        self.command.push(')');
        map_intermediate_sql!(Join, self)
    }
}
