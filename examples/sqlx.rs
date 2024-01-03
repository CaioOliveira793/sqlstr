use std::convert::Infallible;
use std::time::Duration;

use sqlstr::{expr, ArgumentBuffer, SqlCommand};
use sqlx::postgres::PgArguments;
use sqlx::{Arguments, Encode, Postgres, Type};

#[derive(Default)]
struct PostgresArg {
    arg: PgArguments,
    count: u32,
}

impl<T> ArgumentBuffer<T> for PostgresArg
where
    T: for<'q> Encode<'q, Postgres> + Type<Postgres> + Send,
{
    type Error = Infallible;

    fn push(&mut self, value: T) -> Result<(), Self::Error> {
        self.count += 1;
        self.arg.add(value);
        Ok(())
    }

    fn count(&self) -> u32 {
        self.count
    }
}

const DEFAULT_DATABASE_URL: &str = "postgres://root:root@localhost:5432/sqlstr_example";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use sqlx::postgres::PgPoolOptions;
    use sqlx::Row;
    use tokio::runtime;

    let database_url: String = match std::env::args().nth(1) {
        None => DEFAULT_DATABASE_URL.into(),
        Some(arg) => arg
            .as_str()
            .strip_prefix("--database-url=")
            .unwrap_or(DEFAULT_DATABASE_URL)
            .into(),
    };

    let rt = runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()?;

    let query = rt.block_on(async {
        println!("connecting to database at '{}'", database_url);
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&database_url)
            .await
            .expect("connect to database");
        println!("database connected!");

        let query = make_sql_command().expect("create an SQL command as an infallible operation");

        println!("executing query '{}'", query.as_command());
        sqlx::query_with(&query.command, query.arguments.arg)
            .fetch_one(&pool)
            .await
            .expect("execute the SQL command in the connected database")
    });

    println!("query returned!\nextracting values from the query result");

    let num: i32 = query.get(0);
    let list: Vec<String> = query.get(1);
    let boolean: bool = query.get(2);
    let strr: &str = query.get(3);

    println!("asserting returned values ...");
    assert_eq!(num, -1);
    assert_eq!(list, vec!["array", "of", "str"]);
    assert!(!boolean);
    assert_eq!(strr, "sqlstr");

    println!("all values are correct, exiting!");

    Ok(())
}

fn make_sql_command() -> Result<SqlCommand<PostgresArg>, Infallible> {
    let mut sql: SqlCommand<PostgresArg> = SqlCommand::default();

    expr::select(&mut sql);
    expr::separator(&mut sql);
    sql.push_value(-1_i32)?;
    expr::item_separator(&mut sql);
    sql.push_value(["array", "of", "str"])?;
    expr::item_separator(&mut sql);
    sql.push_value(false)?;
    expr::item_separator(&mut sql);
    sql.push_value("sqlstr")?;

    Ok(sql)
}
