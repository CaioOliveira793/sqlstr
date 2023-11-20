use crate::test::*;
use crate::*;

#[test]
fn select_column_table() {
    fn cmd() -> Result<SqlCommand<TestArgs>, Esql> {
        let args = TestArgs::default();
        let sql = select(args)
            .column(User::Id.as_str())?
            .column(User::Created.as_str())?
            .column(User::Name.as_str())?
            .from(User::TABLE)?
            .end();
        Ok(sql)
    }

    let sql = cmd().unwrap();

    assert_eq!(sql.command, "SELECT id, created, name FROM user");
    assert_eq!(sql.arguments.as_str(), "");
    assert_eq!(sql.argument_count, 0);
}

#[test]
fn select_columns_table() {
    fn cmd() -> Result<SqlCommand<TestArgs>, Esql> {
        let args = TestArgs::default();
        let sql = select(args)
            .columns(&User::COLUMNS)?
            .from(User::table())?
            .end();
        Ok(sql)
    }

    let sql = cmd().unwrap();

    assert_eq!(sql.command, "SELECT id, created, name FROM user");
    assert_eq!(sql.arguments.as_str(), "");
    assert_eq!(sql.argument_count, 0);
}

#[test]
fn select_values_iter() {
    fn cmd() -> Result<SqlCommand<TestArgs>, Esql> {
        let args = TestArgs::default();
        let sql = select(args).values([10, -100, 0])?;
        Ok(sql)
    }

    let sql = cmd().unwrap();

    assert_eq!(sql.command, "SELECT $1, $2, $3");
    assert_eq!(sql.arguments.as_str(), "10;-100;0;");
    assert_eq!(sql.argument_count, 3);
}

#[test]
fn select_values() {
    fn cmd() -> Result<SqlCommand<TestArgs>, Esql> {
        let args = TestArgs::default();
        let sql = select(args)
            .value(0)?
            .value("str")?
            .value(true)?
            .value(display_iter(["list", "of", "strings"])?)?
            .end();
        Ok(sql)
    }

    let sql = cmd().unwrap();

    assert_eq!(sql.command, "SELECT $1, $2, $3, $4");
    assert_eq!(sql.arguments.as_str(), "0;str;true;[list,of,strings];");
    assert_eq!(sql.argument_count, 4);
}
