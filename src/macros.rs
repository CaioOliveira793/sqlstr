macro_rules! map_intermediate_sql {
    ($ty:ident, $other:ident) => {
        $ty {
            command: $other.command,
            arguments: $other.arguments,
        }
    };
}

pub(crate) use map_intermediate_sql;
