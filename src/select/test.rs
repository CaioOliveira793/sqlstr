use crate::select::*;
use crate::test::*;
use crate::*;

#[test]
fn select_column_single() {
    fn cmd() -> Result<SqlCommand<TestArgs>, Esql> {
        let args = TestArgs::default();
        let sql = select(args)
            .column(User::Id.as_str())?
            .column(User::Created.as_str())?
            .column(User::Name.as_str())?
            .from_table(User::TABLE)?
            .end();
        Ok(sql)
    }

    let sql = cmd().unwrap();

    assert_eq!(sql.command, "SELECT id, created, name FROM user");
    assert_eq!(sql.arguments.as_str(), "");
}

#[test]
fn select_columns_slice() {
    fn cmd() -> Result<SqlCommand<TestArgs>, Esql> {
        let args = TestArgs::default();
        let sql = select(args)
            .columns(&User::COLUMNS)?
            .from_table(User::table())?
            .end();
        Ok(sql)
    }

    let sql = cmd().unwrap();

    assert_eq!(sql.command, "SELECT id, created, name FROM user");
    assert_eq!(sql.arguments.as_str(), "");
}

#[test]
fn select_static_columns() {
    fn cmd() -> Result<SqlCommand<TestArgs>, Esql> {
        let args = TestArgs::default();
        let sql = select(args)
            .static_columns(columns!("id", "created", "name"))?
            .from_table(User::table())?
            .end();
        Ok(sql)
    }

    let sql = cmd().unwrap();

    assert_eq!(sql.command, "SELECT id, created, name FROM user");
    assert_eq!(sql.arguments.as_str(), "");
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
}

#[test]
fn select_static_from_tables() {
    fn cmd() -> Result<SqlCommand<TestArgs>, Esql> {
        let args = TestArgs::default();
        let sql = select(args)
            .column("name")?
            .static_from_tables(tables!("user", "product"))?
            .end();
        Ok(sql)
    }

    let sql = cmd().unwrap();

    assert_eq!(sql.command, "SELECT name FROM user, product");
    assert_eq!(sql.arguments.as_str(), "");
}
