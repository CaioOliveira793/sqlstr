Create dynamic SQL commands just appending SQL expressions with simple functions.

**sqlstr** provides a set of functions for writing **dynamic SQL** commands with composable expressions. By using just a small abstraction on top of a raw SQL command (usually a [String](alloc::string::String)), it's possible to have **full control** over your SQL and also the **safety** of a simple API.

No need to fiddle with a DSL (Domain-Specific Language) and lose control over your SQL, or concatenate strings manually and make syntactically invalid queries.

# The right abstraction for your SQL

TODO:

## The `WriteSql` trait

TODO:

## Low overhead

TODO:

## Safe parameters in your SQL command

TODO: how `WriteSql::push_value()` works.

## No std

TODO:

# Usage

TODO: explain the `SqlCommand` type, the `ArgumentBuffer` and where to find the helper functions.

## SELECT

```rust
# use core::convert::Infallible;
use sqlstr::{SqlCommand, SqlExpr, Void, sqlexpr, sqlvalue};
use sqlstr::expr::{select, columns_iter, from_table, filter_where, lhs_binary_rhs, Cmp};

# fn main() -> Result<(), Infallible> {
let mut sql: SqlCommand<Void> = SqlCommand::default();

select(&mut sql);
columns_iter(&mut sql, ["name", "email"]);

from_table(&mut sql, "user");

filter_where(&mut sql);
lhs_binary_rhs(
	&mut sql,
	SqlExpr::<&str>::Expr("email"),
	Cmp::Eq,
	SqlExpr::Value("crab@email.com"),
)?;

assert_eq!(sql.as_command(), "SELECT name, email FROM user WHERE email = $1");
# Ok(())
# }
```

## INSERT

```rust
# use core::convert::Infallible;
use sqlstr::{WriteSql, SqlCommand, SqlExpr, Void, sqlexpr, sqlvalue};
use sqlstr::expr::{insert_into, columns_iter, values, item_separator, Group};

# fn main() -> Result<(), Infallible> {
let mut sql: SqlCommand<Void> = SqlCommand::default();

insert_into(&mut sql, "user");

let mut group = Group::open(&mut sql);
columns_iter(&mut group, ["id", "name", "email"]);
group.close();

values(&mut sql);

let mut group = Group::open(&mut sql);
group.push_value(1_u64)?;
item_separator(&mut group);
group.push_value("Rusty")?;
item_separator(&mut group);
group.push_value("rusty@email.com")?;
group.close();

assert_eq!(sql.as_command(), "INSERT INTO user (id, name, email) VALUES ($1, $2, $3)");
# Ok(())
# }
```

## UPDATE

## DELETE

# Feature flags

- `std`: Enables the standard library features (mostly `Error` and `Display` impls)
- `fmt`: Enables the format features from the standard library (`Display` impls)
