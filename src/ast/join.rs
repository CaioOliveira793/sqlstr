/// Comma separated list of shared column names
#[macro_export]
macro_rules! static_using {
    ($first:literal) => {
        concat!("USING (", $first, ")")
    };

    ($first:literal, $($column:literal),* $(,)?) => {
        concat!("USING (", $first, $(", ", $column),*, ")")
    };
}

#[macro_export]
macro_rules! static_join {
    (CROSS $table:literal) => {
        concat!("CROSS JOIN ", $table)
    };

    (INNER $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "INNER JOIN ",
            $table,
            " ON ",
            $crate::ast::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (INNER $table:literal ON NOT $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "INNER JOIN ",
            $table,
            " ON NOT ",
            $crate::ast::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (INNER $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "INNER JOIN ",
            $table,
            " ",
            $crate::ast::static_using!($first, $($column),*),
        )
    };

    (LEFT $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "LEFT JOIN ",
            $table,
            " ON ",
            $crate::ast::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (LEFT $table:literal ON NOT $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "LEFT JOIN ",
            $table,
            " ON NOT ",
            $crate::ast::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (LEFT $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "LEFT JOIN ",
            $table,
            " ",
            $crate::ast::static_using!($first, $($column),*),
        )
    };

    (RIGHT $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "RIGHT JOIN ",
            $table,
            " ON ",
            $crate::ast::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (RIGHT $table:literal ON NOT $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "RIGHT JOIN ",
            $table,
            " ON NOT ",
            $crate::ast::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (RIGHT $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "RIGHT JOIN ",
            $table,
            " ",
            $crate::ast::static_using!($first, $($column),*),
        )
    };

    (FULL $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "FULL JOIN ",
            $table,
            " ON ",
            $crate::ast::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (FULL $table:literal ON NOT $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "FULL JOIN ",
            $table,
            " ON NOT ",
            $crate::ast::static_condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };
    (FULL $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "FULL JOIN ",
            $table,
            " ",
            $crate::ast::static_using!($first, $($column),*)
        )
    };
}

pub use static_join;
pub use static_using;

#[cfg(test)]
mod test {
    #[test]
    fn static_using_macro() {
        assert_eq!(static_using!("id"), "USING (id)");
        assert_eq!(
            static_using!("id", "customer_id"),
            "USING (id, customer_id)"
        );
        assert_eq!(
            static_using!("id", "sale_id", "customer_id",),
            "USING (id, sale_id, customer_id)"
        );
    }

    #[test]
    fn static_join_macro() {
        assert_eq!(static_join!(CROSS "user"), "CROSS JOIN user");

        assert_eq!(
            static_join!(INNER "user" USING ("id", "department")),
            "INNER JOIN user USING (id, department)"
        );
        assert_eq!(
            static_join!(INNER "user" ON "user.id" = "access_history.user_id"),
            "INNER JOIN user ON user.id = access_history.user_id"
        );
        assert_eq!(
            static_join!(INNER "user" ON NOT "user.id" = "access_history.user_id" AND_NOT "user.updated" > "access_history.created"),
            "INNER JOIN user ON NOT user.id = access_history.user_id AND NOT user.updated > access_history.created"
        );

        assert_eq!(
            static_join!(LEFT "user" USING ("id", "department")),
            "LEFT JOIN user USING (id, department)"
        );
        assert_eq!(
            static_join!(LEFT "user" ON "user.id" = "access_history.user_id"),
            "LEFT JOIN user ON user.id = access_history.user_id"
        );
        assert_eq!(
            static_join!(LEFT "user" ON NOT "user.id" = "access_history.user_id"),
            "LEFT JOIN user ON NOT user.id = access_history.user_id"
        );
        assert_eq!(
            static_join!(LEFT "user" ON "user.id" = "access_history.user_id" OR "user.updated" <= "access_history.created"),
            "LEFT JOIN user ON user.id = access_history.user_id OR user.updated <= access_history.created"
        );

        assert_eq!(
            static_join!(RIGHT "user" USING ("id", "department")),
            "RIGHT JOIN user USING (id, department)"
        );
        assert_eq!(
            static_join!(RIGHT "user" ON "user.id" = "access_history.user_id"),
            "RIGHT JOIN user ON user.id = access_history.user_id"
        );
        assert_eq!(
            static_join!(RIGHT "user" ON NOT "user.id" = "access_history.user_id" AND "user.updated" < "access_history.created"),
            "RIGHT JOIN user ON NOT user.id = access_history.user_id AND user.updated < access_history.created"
        );
        assert_eq!(
            static_join!(RIGHT "user" ON "user.id" = "access_history.user_id" AND_NOT "user.updated" >= "access_history.created"),
            "RIGHT JOIN user ON user.id = access_history.user_id AND NOT user.updated >= access_history.created"
        );

        assert_eq!(
            static_join!(FULL "user" USING ("id", "department")),
            "FULL JOIN user USING (id, department)"
        );
        assert_eq!(
            static_join!(FULL "user" ON "user.id" != "access_history.user_id"),
            "FULL JOIN user ON user.id <> access_history.user_id"
        );
        assert_eq!(
            static_join!(FULL "user" ON "user.id" = "access_history.user_id" OR "user.updated" < "access_history.created"),
            "FULL JOIN user ON user.id = access_history.user_id OR user.updated < access_history.created"
        );
        assert_eq!(
            static_join!(FULL "user" ON NOT "user.id" != "access_history.user_id" OR_NOT "user.updated" < "access_history.created"),
            "FULL JOIN user ON NOT user.id <> access_history.user_id OR NOT user.updated < access_history.created"
        );
    }
}
