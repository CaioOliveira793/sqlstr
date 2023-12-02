use super::separator_optional;
use crate::WriteSql;

#[macro_export]
macro_rules! static_tables {
    ($table:literal) => {
        $table
    };

    ($table:literal AS $alias:literal) => {
        concat!($table, " AS ", $alias)
    };

    ($ftable:literal $(AS $falias:literal)?, $($table:literal $(AS $alias:literal)?),* $(,)?) => {
        concat!($ftable $(, " AS ", $falias)?, $(", ", $table $(, " AS ", $alias)?),*)
    };
}

#[macro_export]
macro_rules! static_from_tables {
    (FROM $table:literal) => {
        concat!("FROM ", $table)
    };

    (FROM $table:literal AS $alias:literal) => {
        concat!("FROM ", $table, " AS ", $alias)
    };

    (FROM $ftable:literal $(AS $falias:literal)?, $($table:literal $(AS $alias:literal)?),* $(,)?) => {
        concat!("FROM ", $ftable $(, " AS ", $falias)?, $(", ", $table $(, " AS ", $alias)?),*)
    };
}

pub fn from_tables<'t, Sql, Arg, I>(sql: &mut Sql, tables: I)
where
    Sql: WriteSql<Arg>,
    I: IntoIterator<Item = &'t str>,
{
    separator_optional(sql);
    sql.push_cmd("FROM");

    let mut tbls = tables.into_iter();
    if let Some(tbl) = tbls.next() {
        sql.push_cmd(" ");
        sql.push_cmd(tbl);
    }
    for tbl in tbls {
        sql.push_cmd(", ");
        sql.push_cmd(tbl);
    }
}

#[cfg(test)]
mod test {
    use super::from_tables;
    use crate::{
        expr::{select, separator},
        test::TestArgs,
        SqlCommand,
    };

    #[test]
    fn from_tables_no_tables() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        from_tables(&mut sql, []);

        assert_eq!(sql.as_command(), "FROM");
    }

    #[test]
    fn from_tables_one_table() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        from_tables(&mut sql, ["user"]);

        assert_eq!(sql.as_command(), "FROM user");
    }

    #[test]
    fn from_tables_multiple_table() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        from_tables(&mut sql, ["user", "access_history"]);

        assert_eq!(sql.as_command(), "FROM user, access_history");
    }

    #[test]
    fn static_tables_test() {
        assert_eq!(static_tables!("user"), "user");
        assert_eq!(static_tables!("user" AS "u"), "user AS u");
        assert_eq!(static_tables!("'quoted table'",), "'quoted table'");
        assert_eq!(
            static_tables!("'quoted table'" AS "'other'",),
            "'quoted table' AS 'other'"
        );
        assert_eq!(static_tables!("user", "customer"), "user, customer");
        assert_eq!(
            static_tables!("user", "customer", "organization",),
            "user, customer, organization"
        );
        assert_eq!(
            static_tables!("user", "customer" AS "c", "organization", "product" AS "p",),
            "user, customer AS c, organization, product AS p"
        );
    }

    #[test]
    fn static_from_tables_test() {
        assert_eq!(static_from_tables!(FROM "user"), "FROM user");
        assert_eq!(static_from_tables!(FROM "user" AS "u"), "FROM user AS u");
        assert_eq!(
            static_from_tables!(FROM "'quoted table'",),
            "FROM 'quoted table'"
        );
        assert_eq!(
            static_from_tables!(FROM "'quoted table'" AS "'other'",),
            "FROM 'quoted table' AS 'other'"
        );
        assert_eq!(
            static_from_tables!(FROM "user", "customer"),
            "FROM user, customer"
        );
        assert_eq!(
            static_from_tables!(FROM "user", "customer", "organization",),
            "FROM user, customer, organization"
        );
        assert_eq!(
            static_from_tables!(FROM "user", "customer" AS "c", "organization", "product" AS "p",),
            "FROM user, customer AS c, organization, product AS p"
        );
    }

    #[test]
    fn select_static_from_tables() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

        select(&mut sql);
        separator(&mut sql);
        sql.push_cmd("name");
        separator(&mut sql);
        sql.push_cmd(static_from_tables!(FROM "user", "product"));

        assert_eq!(sql.command, "SELECT name FROM user, product");
        assert_eq!(sql.arguments.as_str(), "");
    }
}
