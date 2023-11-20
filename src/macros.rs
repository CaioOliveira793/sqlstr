#[macro_export]
macro_rules! display_sql_command {
    ($ty:ident) => {
        #[cfg(feature = "fmt")]
        impl<Arg> core::fmt::Display for $ty<Arg> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.command.fmt(f)
            }
        }
    };
}

#[macro_export]
macro_rules! map_intermediate_sql {
    ($ty:ident, $other:ident) => {
        $ty {
            command: $other.command,
            argument_count: $other.argument_count,
            arguments: $other.arguments,
        }
    };
}
