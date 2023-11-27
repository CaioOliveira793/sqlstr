use alloc::collections::TryReserveError;

#[cfg(feature = "std")]
use std::error::Error;

#[cfg_attr(any(feature = "fmt", test), derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub enum SqlError<EArg> {
    CommandBuffer(TryReserveError),
    Argument(EArg),
    ArgumentNotFound,
}

impl<ArgErr> From<TryReserveError> for SqlError<ArgErr> {
    fn from(value: TryReserveError) -> Self {
        Self::CommandBuffer(value)
    }
}

#[cfg(any(feature = "fmt", test))]
impl<ArgErr> core::fmt::Display for SqlError<ArgErr> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CommandBuffer(_) => f.write_str("sql command"),
            Self::Argument(_) => f.write_str("sql argument"),
            Self::ArgumentNotFound => f.write_str("sql argument not found"),
        }
    }
}

#[cfg(feature = "std")]
impl<ArgErr> Error for SqlError<ArgErr>
where
    ArgErr: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CommandBuffer(err) => Some(err as &(dyn Error + 'static)),
            Self::Argument(err) => Some(err as &(dyn Error + 'static)),
            Self::ArgumentNotFound => None,
        }
    }
}
