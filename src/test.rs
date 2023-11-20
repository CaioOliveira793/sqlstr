use alloc::string::String;
use core::fmt::{self, Display, Write};
use core::ops::Deref;

use crate::error::SqlError;
use crate::ArgumentBuffer;

pub type Esql = SqlError<fmt::Error>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TestArgs(String);

impl TestArgs {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for TestArgs {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl<T> ArgumentBuffer<T> for TestArgs
where
    T: Display,
{
    type Error = fmt::Error;

    fn push(&mut self, value: T) -> Result<(), Self::Error> {
        self.0.write_fmt(format_args!("{value};"))
    }
}

pub fn display_iter<'a, I, T>(iter: I) -> Result<String, SqlError<fmt::Error>>
where
    I: IntoIterator<Item = &'a T>,
    T: Display + ?Sized + 'a,
{
    let mut buffer = String::new();
    buffer.push('[');
    for item in iter {
        buffer
            .write_fmt(format_args!("{item},"))
            .map_err(SqlError::Argument)?;
    }
    buffer.push(']');
    Ok(buffer)
}

pub enum User {
    Id,
    Created,
    Name,
}

impl User {
    pub const TABLE: &str = "user";

    pub const COLUMNS: [&str; 3] = [
        Self::Id.as_str(),
        Self::Created.as_str(),
        Self::Name.as_str(),
    ];

    pub const fn table() -> &'static str {
        Self::TABLE
    }

    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Id => "id",
            Self::Created => "created",
            Self::Name => "name",
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
