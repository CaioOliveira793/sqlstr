use alloc::string::String;

#[cfg(feature = "fmt")]
use core::fmt::{self, Display};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SqlCommand<Arg> {
    pub command: String,
    pub arguments: Arg,
    pub argument_count: u32,
}

impl<Arg> SqlCommand<Arg> {
    pub const fn new(command: String, arguments: Arg, argument_count: u32) -> Self {
        Self {
            command,
            arguments,
            argument_count,
        }
    }
}

#[cfg(feature = "fmt")]
impl<Arg> Display for SqlCommand<Arg> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.command.fmt(f)
    }
}

pub trait ArgumentBuffer<T> {
    type Error;

    fn push(&mut self, value: T) -> Result<(), Self::Error>;
}
