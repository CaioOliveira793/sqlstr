#[allow(unused_macros)]
macro_rules! static_comparison {
    (=) => {
        "="
    };
    (!=) => {
        "<>"
    };
    (>) => {
        ">"
    };
    (<) => {
        "<"
    };
    (>=) => {
        ">="
    };
    (<=) => {
        "<="
    };
}

#[allow(unused_macros)]
macro_rules! static_logical_op {
    (NOT) => {
        "NOT"
    };
    (AND) => {
        "AND"
    };
    (OR) => {
        "OR"
    };
    (AND_NOT) => {
        "AND NOT"
    };
    (OR_NOT) => {
        "OR NOT"
    };
}

#[allow(unused_imports)]
pub(super) use static_comparison;

#[allow(unused_imports)]
pub(super) use static_logical_op;

#[macro_export]
macro_rules! static_condition {
    ($a:literal $op:tt $b:literal) => {
        concat!(
            $a,
            " ",
            $crate::expr::static_comparison!($op),
            " ",
            $b
        )
    };
    ($pre_logic:tt $a:literal $op:tt $b:literal) => {
        concat!(
            $crate::expr::static_logical_op!($pre_logic),
            " ",
            $a,
            " ",
            $crate::expr::static_comparison!($op),
            " ",
            $b
        )
    };

    ($a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)+) => {
        concat!(
            $crate::expr::static_condition!($a $op $b),
            $(
                " ",
                $crate::expr::static_logical_op!($logic_op),
                " ",
                $crate::expr::static_condition!($ax $opx $bx)
            )+
        )
    };
    ($pre_logic:tt $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)+) => {
        concat!(
            $crate::expr::static_logical_op!($pre_logic),
            " ",
            $crate::expr::static_condition!($a $op $b),
            $(
                " ",
                $crate::expr::static_logical_op!($logic_op),
                " ",
                $crate::expr::static_condition!($ax $opx $bx)
            )+
        )
    };
}

pub use static_condition;

#[cfg(test)]
mod test {

    #[test]
    fn static_condition_macro() {
        assert_eq!(
            static_condition!(NOT "access_history.user_id" = "user.id"),
            "NOT access_history.user_id = user.id"
        );
        assert_eq!(
            static_condition!("access_history.user_id" > "user.id"),
            "access_history.user_id > user.id"
        );
        assert_eq!(
            static_condition!(AND_NOT "access_history.user_id" >= "user.id"),
            "AND NOT access_history.user_id >= user.id"
        );
        assert_eq!(
            static_condition!("user.id" = "access_history.user_id" OR "user.updated" < "access_history.created"),
            "user.id = access_history.user_id OR user.updated < access_history.created"
        );
        assert_eq!(
            static_condition!("user.id" != "access_history.user_id"),
            "user.id <> access_history.user_id"
        );
        assert_eq!(
            static_condition!(NOT "user.id" = "access_history.user_id" AND_NOT "user.updated" < "access_history.created"),
            "NOT user.id = access_history.user_id AND NOT user.updated < access_history.created"
        )
    }
}
