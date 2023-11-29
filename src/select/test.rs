use crate::test::{display_iter, TestArgs};
use crate::*;

#[test]
fn ast_select_values_iter() {
    // let values = [10, -100, 0];
    let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

    ast::select(&mut sql);
    ast::separator_optional(&mut sql);

    // TODO: create `fn ast::values_iter()`
    // to push values into the WriteSql, verifying for the first value if it is a value or a command.
    sql.push_value(10).unwrap();
    ast::item_separator(&mut sql);
    sql.push_value(-100).unwrap();
    ast::item_separator(&mut sql);
    sql.push_value(0).unwrap();

    assert_eq!(sql.command, "SELECT $1, $2, $3");
    assert_eq!(sql.arguments.as_str(), "10;-100;0;");
}

#[test]
fn ast_select_values() {
    let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

    ast::select(&mut sql);
    ast::separator_optional(&mut sql);
    sql.push_value(0).unwrap();
    ast::item_separator(&mut sql);
    sql.push_value("str").unwrap();
    ast::item_separator(&mut sql);
    sql.push_value(true).unwrap();
    ast::item_separator(&mut sql);
    sql.push_value(display_iter(["list", "of", "strings"]).unwrap())
        .unwrap();

    assert_eq!(sql.as_command(), "SELECT $1, $2, $3, $4");
    assert_eq!(sql.arguments.as_str(), "0;str;true;[list,of,strings];");
}

#[test]
fn ast_select_static_from_tables() {
    let mut sql: SqlCommand<TestArgs> = SqlCommand::default();

    ast::select(&mut sql);
    ast::separator_optional(&mut sql);
    sql.push_cmd("name");
    ast::separator_optional(&mut sql);
    sql.push_cmd("FROM ");
    sql.push_cmd(static_tables!("user", "product"));

    assert_eq!(sql.command, "SELECT name FROM user, product");
    assert_eq!(sql.arguments.as_str(), "");
}
