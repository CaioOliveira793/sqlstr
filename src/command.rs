use alloc::string::String;

use crate::macros::display_sql_command;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SqlCommand<Arg> {
    pub command: String,
    pub arguments: Arg,
}

impl<Arg> SqlCommand<Arg> {
    pub const fn new(command: String, arguments: Arg) -> Self {
        Self { command, arguments }
    }
}

display_sql_command!(SqlCommand);

pub trait ArgumentBuffer<T> {
    type Error;

    fn push(&mut self, value: T) -> Result<(), Self::Error>;

    /// Returns the number of arguments pushed into `Self`
    fn count(&self) -> u32;
}
