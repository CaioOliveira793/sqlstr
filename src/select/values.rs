use alloc::string::String;

use crate::{
    error::SqlError, format_num::format_u32_base10, macros::map_intermediate_sql, ArgumentBuffer,
    SqlCommand,
};

pub struct SelectValue<Arg> {
    pub(super) command: String,
    pub(super) arguments: Arg,
}

impl<Arg> SelectValue<Arg> {
    pub(super) fn transition_value<T>(
        mut self,
        value: T,
    ) -> Result<Self, SqlError<<Arg as ArgumentBuffer<T>>::Error>>
    where
        Arg: ArgumentBuffer<T>,
    {
        self.arguments.push(value).map_err(SqlError::Argument)?;

        let mut buf = [0; 10];
        self.command.push_str(" $");
        self.command
            .push_str(format_u32_base10(self.arguments.count(), &mut buf));

        Ok(self)
    }

    pub fn value<T>(mut self, value: T) -> Result<Self, SqlError<<Arg as ArgumentBuffer<T>>::Error>>
    where
        Arg: ArgumentBuffer<T>,
    {
        self.command.push(',');
        self.transition_value(value)
    }

    pub(super) fn transition_values<T, I>(
        mut self,
        values: I,
    ) -> Result<Self, SqlError<<Arg as ArgumentBuffer<T>>::Error>>
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

        Ok(self)
    }

    pub fn values<T, I>(
        mut self,
        values: I,
    ) -> Result<Self, SqlError<<Arg as ArgumentBuffer<T>>::Error>>
    where
        Arg: ArgumentBuffer<T>,
        I: IntoIterator<Item = T>,
    {
        self.command.push(',');
        self.transition_values(values)
    }

    pub fn end(self) -> SqlCommand<Arg> {
        map_intermediate_sql!(SqlCommand, self)
    }
}
