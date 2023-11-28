use super::separator_optional;
use crate::WriteSql;

/// Row-level lock strength
///
/// - [Postgres row-level locks](https://www.postgresql.org/docs/current/explicit-locking.html#LOCKING-ROWS)
/// - [MySQL locking reads](https://dev.mysql.com/doc/refman/8.2/en/innodb-locking-reads.html)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RowLockStrength {
    Update,
    NoKeyUpdate,
    Share,
    KeyShare,
}

impl RowLockStrength {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Update => "UPDATE",
            Self::NoKeyUpdate => "NO KEY UPDATE",
            Self::Share => "SHARE",
            Self::KeyShare => "KEY SHARE",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RowLockConcurrency {
    NoWait,
    SkipLocked,
}

impl RowLockConcurrency {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::NoWait => "NOWAIT",
            Self::SkipLocked => "SKIP LOCKED",
        }
    }
}

pub fn row_lock<'t, Sql, Arg, I>(
    sql: &mut Sql,
    strength: RowLockStrength,
    tables: I,
    concurrency: Option<RowLockConcurrency>,
) where
    Sql: WriteSql<Arg>,
    I: IntoIterator<Item = &'t str>,
{
    separator_optional(sql);
    sql.push_cmd("FOR ");
    sql.push_cmd(strength.as_str());

    let mut tables = tables.into_iter();
    if let Some(table) = tables.next() {
        sql.push_cmd(" OF ");
        sql.push_cmd(table);
    }
    for table in tables {
        sql.push_cmd(", ");
        sql.push_cmd(table);
    }

    if let Some(concurr) = concurrency {
        sql.push_cmd(" ");
        sql.push_cmd(concurr.as_str());
    }
}

/// Table-level lock mode
///
/// [Postgres table-level locks](https://www.postgresql.org/docs/current/explicit-locking.html#LOCKING-TABLES)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TableLockMode {
    AccessShare,
    RowShare,
    RowExclusive,
    ShareUpdateExclusive,
    Share,
    ShareRowExclusive,
    Exclusive,
    AccessExclusive,
}

impl TableLockMode {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::AccessShare => "ACCESS SHARE",
            Self::RowShare => "ROW SHARE",
            Self::RowExclusive => "ROW EXCLUSIVE",
            Self::ShareUpdateExclusive => "SHARE UPDATE EXCLUSIVE",
            Self::Share => "SHARE",
            Self::ShareRowExclusive => "SHARE ROW EXCLUSIVE",
            Self::Exclusive => "EXCLUSIVE",
            Self::AccessExclusive => "ACCESS EXCLUSIVE",
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum TableLock<'t> {
    Only(&'t str),
    Tables(&'t [&'t str]),
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum TableLockWait {
    NoWait,
    #[default]
    None,
}

impl TableLockWait {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::NoWait => "NOWAIT",
            Self::None => "",
        }
    }
}

pub fn table_lock<Sql, Arg>(
    sql: &mut Sql,
    tables: TableLock<'_>,
    lock_mode: Option<TableLockMode>,
    nowait: TableLockWait,
) where
    Sql: WriteSql<Arg>,
{
    separator_optional(sql);
    sql.push_cmd("LOCK TABLE ");

    match tables {
        TableLock::Only(table) => {
            sql.push_cmd("ONLY ");
            sql.push_cmd(table);
        }
        TableLock::Tables(tbls) => {
            if let Some(tbl) = tbls.first() {
                sql.push_cmd(tbl);
            }
            for tbl in &tbls[1..] {
                sql.push_cmd(", ");
                sql.push_cmd(tbl);
            }
        }
    }

    if let Some(mode) = lock_mode {
        sql.push_cmd(" ");
        sql.push_cmd(mode.as_str());
    }

    if nowait == TableLockWait::NoWait {
        sql.push_cmd(" NOWAIT");
    }
}

#[allow(unused_macros)]
macro_rules! lock_strength {
    (UPDATE) => {
        "UPDATE"
    };
    (NO_KEY_UPDATE) => {
        "NO KEY UPDATE"
    };
    (SHARE) => {
        "SHARE"
    };
    (KEY_SHARE) => {
        "KEY SHARE"
    };
}

#[allow(unused)]
pub(super) use lock_strength;

/// Row-level lock
///
/// - [Postgres row-level locks](https://www.postgresql.org/docs/current/explicit-locking.html#LOCKING-ROWS)
#[macro_export]
macro_rules! static_row_lock {
    (FOR $s1:tt) => {
        concat!("FOR ", $crate::ast::lock_strength!($s1))
    };
    (FOR $s1:tt NOWAIT) => {
        concat!("FOR ", $crate::ast::lock_strength!($s1), " NOWAIT")
    };
    (FOR $s1:tt SKIP LOCKED) => {
        concat!("FOR ", $crate::ast::lock_strength!($s1), " SKIP LOCKED")
    };

    (FOR $s1:tt OF $ftbl:literal$(,)? $($tbl:literal),*) => {
        concat!(
            "FOR ",
            $crate::ast::lock_strength!($s1),
            " OF ",
            $ftbl,
            $(", ", $tbl),*
        )
    };
    (FOR $s1:tt OF $ftbl:literal$(,)? $($tbl:literal),* NOWAIT) => {
        concat!(
            "FOR ",
            $crate::ast::lock_strength!($s1),
            " OF ",
            $ftbl,
            $(", ", $tbl,)*
            " NOWAIT"
        )
    };
    (FOR $s1:tt OF $ftbl:literal$(,)? $($tbl:literal),* SKIP LOCKED) => {
        concat!(
            "FOR ",
            $crate::ast::lock_strength!($s1),
            " OF ",
            $ftbl,
            $(", ", $tbl,)*
            " SKIP LOCKED"
        )
    };
}

pub use static_row_lock;

#[allow(unused_macros)]
macro_rules! lockmode {
    (ACCESS_SHARE) => {
        "ACCESS SHARE"
    };
    (ROW_SHARE) => {
        "ROW SHARE"
    };
    (ROW_EXCLUSIVE) => {
        "ROW EXCLUSIVE"
    };
    (SHARE_UPDATE_EXCLUSIVE) => {
        "SHARE UPDATE EXCLUSIVE"
    };
    (SHARE) => {
        "SHARE"
    };
    (SHARE_ROW_EXCLUSIVE) => {
        "SHARE ROW EXCLUSIVE"
    };
    (EXCLUSIVE) => {
        "EXCLUSIVE"
    };
    (ACCESS_EXCLUSIVE) => {
        "ACCESS EXCLUSIVE"
    };
}

#[allow(unused)]
pub(super) use lockmode;

/// LOCK sql command
///
/// - [Postgres table lock](https://www.postgresql.org/docs/current/sql-lock.html)
#[macro_export]
macro_rules! static_table_lock {
    (LOCK TABLE $table:literal$(,)? $($tables:literal),*) => {
        concat!("LOCK TABLE ", $table, $(", ", $tables,)*)
    };
    (LOCK TABLE ONLY $table:literal) => {
        concat!("LOCK TABLE ONLY ", $table)
    };
    (LOCK TABLE $table:literal$(,)? $($tables:literal),* NOWAIT) => {
        concat!("LOCK TABLE ", $table, $(", ", $tables,)* " NOWAIT")
    };
    (LOCK TABLE ONLY $table:literal NOWAIT) => {
        concat!("LOCK TABLE ONLY ", $table, " NOWAIT")
    };

    (LOCK TABLE $table:literal$(,)? $($tables:literal),* IN $mode:tt MODE) => {
        concat!("LOCK TABLE ", $table, $(", ", $tables,)* " IN ", $crate::ast::lockmode!($mode), " MODE")
    };
	(LOCK TABLE ONLY $table:literal IN $mode:tt MODE) => {
        concat!("LOCK TABLE ONLY ", $table, " IN ", $crate::ast::lockmode!($mode), " MODE")
    };
    (LOCK TABLE $table:literal$(,)? $($tables:literal),* IN $mode:tt MODE NOWAIT) => {
        concat!("LOCK TABLE ", $table, $(", ", $tables,)* " IN ", $crate::ast::lockmode!($mode), " MODE NOWAIT")
    };
	(LOCK TABLE ONLY $table:literal IN $mode:tt MODE NOWAIT) => {
        concat!("LOCK TABLE ONLY ", $table, " IN ", $crate::ast::lockmode!($mode), " MODE NOWAIT")
    };
}

pub use static_table_lock;

#[cfg(test)]
mod row_lock_test {
    use super::{row_lock, RowLockStrength};
    use crate::{ast::RowLockConcurrency, SqlCommand, Void};

    #[test]
    fn row_locking_strength() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        row_lock(&mut sql, RowLockStrength::NoKeyUpdate, [], None);

        assert_eq!(sql.as_command(), "FOR NO KEY UPDATE");
    }

    #[test]
    fn row_locking_strength_with_concurrency() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        row_lock(
            &mut sql,
            RowLockStrength::Share,
            [],
            Some(RowLockConcurrency::NoWait),
        );

        assert_eq!(sql.as_command(), "FOR SHARE NOWAIT");
    }

    #[test]
    fn row_locking_with_tables() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        row_lock(
            &mut sql,
            RowLockStrength::Update,
            ["user", "access_history"],
            None,
        );

        assert_eq!(sql.as_command(), "FOR UPDATE OF user, access_history");
    }

    #[test]
    fn row_locking_with_tables_and_concurrency() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        row_lock(
            &mut sql,
            RowLockStrength::KeyShare,
            ["customer"],
            Some(RowLockConcurrency::SkipLocked),
        );

        assert_eq!(sql.as_command(), "FOR KEY SHARE OF customer SKIP LOCKED");
    }
}

#[cfg(test)]
mod table_lock_test {
    use crate::{
        ast::{table_lock, TableLock, TableLockMode, TableLockWait},
        SqlCommand, Void,
    };

    #[test]
    fn lock_table_only() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        table_lock(&mut sql, TableLock::Only("user"), None, TableLockWait::None);

        assert_eq!(sql.as_command(), "LOCK TABLE ONLY user");
    }

    #[test]
    fn lock_table_only_with_lockmode() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        table_lock(
            &mut sql,
            TableLock::Only("user"),
            Some(TableLockMode::AccessExclusive),
            TableLockWait::None,
        );

        assert_eq!(sql.as_command(), "LOCK TABLE ONLY user ACCESS EXCLUSIVE");
    }

    #[test]
    fn lock_table_only_with_lockmode_and_wait() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        table_lock(
            &mut sql,
            TableLock::Only("user"),
            Some(TableLockMode::AccessExclusive),
            TableLockWait::NoWait,
        );

        assert_eq!(
            sql.as_command(),
            "LOCK TABLE ONLY user ACCESS EXCLUSIVE NOWAIT"
        );
    }

    #[test]
    fn lock_table_only_with_onwait() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        table_lock(
            &mut sql,
            TableLock::Only("user"),
            None,
            TableLockWait::NoWait,
        );

        assert_eq!(sql.as_command(), "LOCK TABLE ONLY user NOWAIT");
    }

    #[test]
    fn lock_tables() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        table_lock(
            &mut sql,
            TableLock::Tables(&["user", "customer"]),
            None,
            TableLockWait::None,
        );

        assert_eq!(sql.as_command(), "LOCK TABLE user, customer");
    }

    #[test]
    fn lock_tables_with_lockmode() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        table_lock(
            &mut sql,
            TableLock::Tables(&["user", "customer"]),
            Some(TableLockMode::Share),
            TableLockWait::None,
        );

        assert_eq!(sql.as_command(), "LOCK TABLE user, customer SHARE");
    }

    #[test]
    fn lock_tables_with_lockmode_and_nowait() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        table_lock(
            &mut sql,
            TableLock::Tables(&["user", "customer"]),
            Some(TableLockMode::RowExclusive),
            TableLockWait::NoWait,
        );

        assert_eq!(
            sql.as_command(),
            "LOCK TABLE user, customer ROW EXCLUSIVE NOWAIT"
        );
    }

    #[test]
    fn lock_tables_with_nowait() {
        let mut sql: SqlCommand<Void> = SqlCommand::default();

        table_lock(
            &mut sql,
            TableLock::Tables(&["user", "customer"]),
            None,
            TableLockWait::NoWait,
        );

        assert_eq!(sql.as_command(), "LOCK TABLE user, customer NOWAIT");
    }
}

#[cfg(test)]
mod static_row_lock_test {
    #[test]
    fn static_row_lock_macro() {
        assert_eq!(static_row_lock!(FOR UPDATE), "FOR UPDATE");
        assert_eq!(static_row_lock!(FOR NO_KEY_UPDATE), "FOR NO KEY UPDATE");
        assert_eq!(static_row_lock!(FOR SHARE), "FOR SHARE");
        assert_eq!(static_row_lock!(FOR KEY_SHARE), "FOR KEY SHARE");

        assert_eq!(static_row_lock!(FOR UPDATE NOWAIT), "FOR UPDATE NOWAIT");
        assert_eq!(
            static_row_lock!(FOR NO_KEY_UPDATE NOWAIT),
            "FOR NO KEY UPDATE NOWAIT"
        );
        assert_eq!(static_row_lock!(FOR SHARE NOWAIT), "FOR SHARE NOWAIT");
        assert_eq!(
            static_row_lock!(FOR KEY_SHARE NOWAIT),
            "FOR KEY SHARE NOWAIT"
        );

        assert_eq!(
            static_row_lock!(FOR UPDATE SKIP LOCKED),
            "FOR UPDATE SKIP LOCKED"
        );
        assert_eq!(
            static_row_lock!(FOR NO_KEY_UPDATE SKIP LOCKED),
            "FOR NO KEY UPDATE SKIP LOCKED"
        );
        assert_eq!(
            static_row_lock!(FOR SHARE SKIP LOCKED),
            "FOR SHARE SKIP LOCKED"
        );
        assert_eq!(
            static_row_lock!(FOR KEY_SHARE SKIP LOCKED),
            "FOR KEY SHARE SKIP LOCKED"
        );

        assert_eq!(static_row_lock!(FOR UPDATE OF "user"), "FOR UPDATE OF user");
        assert_eq!(
            static_row_lock!(FOR NO_KEY_UPDATE OF "user"),
            "FOR NO KEY UPDATE OF user"
        );
        assert_eq!(static_row_lock!(FOR SHARE OF "user"), "FOR SHARE OF user");
        assert_eq!(
            static_row_lock!(FOR KEY_SHARE OF "user"),
            "FOR KEY SHARE OF user"
        );

        assert_eq!(
            static_row_lock!(FOR UPDATE OF "user" NOWAIT),
            "FOR UPDATE OF user NOWAIT"
        );
        assert_eq!(
            static_row_lock!(FOR NO_KEY_UPDATE OF "user" NOWAIT),
            "FOR NO KEY UPDATE OF user NOWAIT"
        );
        assert_eq!(
            static_row_lock!(FOR SHARE OF "user" NOWAIT),
            "FOR SHARE OF user NOWAIT"
        );
        assert_eq!(
            static_row_lock!(FOR KEY_SHARE OF "user" NOWAIT),
            "FOR KEY SHARE OF user NOWAIT"
        );

        assert_eq!(
            static_row_lock!(FOR UPDATE OF "user" SKIP LOCKED),
            "FOR UPDATE OF user SKIP LOCKED"
        );
        assert_eq!(
            static_row_lock!(FOR NO_KEY_UPDATE OF "user" SKIP LOCKED),
            "FOR NO KEY UPDATE OF user SKIP LOCKED"
        );
        assert_eq!(
            static_row_lock!(FOR SHARE OF "user" SKIP LOCKED),
            "FOR SHARE OF user SKIP LOCKED"
        );
        assert_eq!(
            static_row_lock!(FOR KEY_SHARE OF "user" SKIP LOCKED),
            "FOR KEY SHARE OF user SKIP LOCKED"
        );

        assert_eq!(
            static_row_lock!(FOR UPDATE OF "user", "access", "customer"),
            "FOR UPDATE OF user, access, customer"
        );
        assert_eq!(
            static_row_lock!(FOR NO_KEY_UPDATE OF "user", "access", "customer"),
            "FOR NO KEY UPDATE OF user, access, customer"
        );
        assert_eq!(
            static_row_lock!(FOR SHARE OF "user", "access", "customer"),
            "FOR SHARE OF user, access, customer"
        );
        assert_eq!(
            static_row_lock!(FOR KEY_SHARE OF "user", "access", "customer"),
            "FOR KEY SHARE OF user, access, customer"
        );

        assert_eq!(
            static_row_lock!(FOR UPDATE OF "user", "access", "customer" NOWAIT),
            "FOR UPDATE OF user, access, customer NOWAIT"
        );
        assert_eq!(
            static_row_lock!(FOR NO_KEY_UPDATE OF "user", "access", "customer" NOWAIT),
            "FOR NO KEY UPDATE OF user, access, customer NOWAIT"
        );
        assert_eq!(
            static_row_lock!(FOR SHARE OF "user", "access", "customer" NOWAIT),
            "FOR SHARE OF user, access, customer NOWAIT"
        );
        assert_eq!(
            static_row_lock!(FOR KEY_SHARE OF "user", "access", "customer" NOWAIT),
            "FOR KEY SHARE OF user, access, customer NOWAIT"
        );

        assert_eq!(
            static_row_lock!(FOR UPDATE OF "user", "access", "customer" SKIP LOCKED),
            "FOR UPDATE OF user, access, customer SKIP LOCKED"
        );
        assert_eq!(
            static_row_lock!(FOR NO_KEY_UPDATE OF "user", "access", "customer" SKIP LOCKED),
            "FOR NO KEY UPDATE OF user, access, customer SKIP LOCKED"
        );
        assert_eq!(
            static_row_lock!(FOR SHARE OF "user", "access", "customer" SKIP LOCKED),
            "FOR SHARE OF user, access, customer SKIP LOCKED"
        );
        assert_eq!(
            static_row_lock!(FOR KEY_SHARE OF "user", "access", "customer" SKIP LOCKED),
            "FOR KEY SHARE OF user, access, customer SKIP LOCKED"
        );
    }

    #[test]
    fn static_table_lock_macro() {
        assert_eq!(static_table_lock!(LOCK TABLE "user"), "LOCK TABLE user");
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record"),
            "LOCK TABLE user, customer, access_record"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user" IN ROW_EXCLUSIVE MODE),
            "LOCK TABLE user IN ROW EXCLUSIVE MODE"
        );

        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN ACCESS_SHARE MODE),
            "LOCK TABLE user, customer, access_record IN ACCESS SHARE MODE"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN ROW_SHARE MODE),
            "LOCK TABLE user, customer, access_record IN ROW SHARE MODE"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN ROW_EXCLUSIVE MODE),
            "LOCK TABLE user, customer, access_record IN ROW EXCLUSIVE MODE"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN SHARE_UPDATE_EXCLUSIVE MODE),
            "LOCK TABLE user, customer, access_record IN SHARE UPDATE EXCLUSIVE MODE"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN SHARE MODE),
            "LOCK TABLE user, customer, access_record IN SHARE MODE"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN SHARE_ROW_EXCLUSIVE MODE),
            "LOCK TABLE user, customer, access_record IN SHARE ROW EXCLUSIVE MODE"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN EXCLUSIVE MODE),
            "LOCK TABLE user, customer, access_record IN EXCLUSIVE MODE"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN ACCESS_EXCLUSIVE MODE),
            "LOCK TABLE user, customer, access_record IN ACCESS EXCLUSIVE MODE"
        );

        assert_eq!(
            static_table_lock!(LOCK TABLE "user" NOWAIT),
            "LOCK TABLE user NOWAIT"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" NOWAIT),
            "LOCK TABLE user, customer, access_record NOWAIT"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user" IN ROW_EXCLUSIVE MODE NOWAIT),
            "LOCK TABLE user IN ROW EXCLUSIVE MODE NOWAIT"
        );

        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "access_record" IN ACCESS_SHARE MODE NOWAIT),
            "LOCK TABLE user, access_record IN ACCESS SHARE MODE NOWAIT"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "customer", "access_record" IN ROW_SHARE MODE NOWAIT),
            "LOCK TABLE customer, access_record IN ROW SHARE MODE NOWAIT"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer" IN ROW_EXCLUSIVE MODE NOWAIT),
            "LOCK TABLE user, customer IN ROW EXCLUSIVE MODE NOWAIT"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN SHARE_UPDATE_EXCLUSIVE MODE NOWAIT),
            "LOCK TABLE user, customer, access_record IN SHARE UPDATE EXCLUSIVE MODE NOWAIT"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer" IN SHARE MODE NOWAIT),
            "LOCK TABLE user, customer IN SHARE MODE NOWAIT"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "access_record" IN SHARE_ROW_EXCLUSIVE MODE NOWAIT),
            "LOCK TABLE access_record IN SHARE ROW EXCLUSIVE MODE NOWAIT"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN EXCLUSIVE MODE NOWAIT),
            "LOCK TABLE user, customer, access_record IN EXCLUSIVE MODE NOWAIT"
        );
        assert_eq!(
            static_table_lock!(LOCK TABLE "user", "customer", "access_record" IN ACCESS_EXCLUSIVE MODE NOWAIT),
            "LOCK TABLE user, customer, access_record IN ACCESS EXCLUSIVE MODE NOWAIT"
        );
    }
}
