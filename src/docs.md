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

## Composition

TODO: show how complex queries can be abstracted with functions.

```rust
# use core::convert::Infallible;
use sqlstr::{WriteSql, ArgumentBuffer, SqlCommand, SqlExpr, Void, sqlexpr, sqlvalue};
use sqlstr::expr::{select, column, from_table, filter_where, lhs_binary_rhs, Cmp};

fn select_user_by_id<Sql, Arg>(
	sql: &mut Sql,
	id: u64
) -> Result<(), <Arg as ArgumentBuffer<u64>>::Error>
where
	Sql: WriteSql<Arg>,
	Arg: ArgumentBuffer<u64>,
{
	select(sql);
	column(sql, "*");

	from_table(sql, "user");

	filter_where(sql);
	lhs_binary_rhs(
		sql,
		SqlExpr::<u64>::Expr("id"),
		Cmp::Eq,
		SqlExpr::Value(id),
	)?;

	Ok(())
}

# fn main() -> Result<(), Infallible> {
let mut sql: SqlCommand<Void> = SqlCommand::default();

select_user_by_id(&mut sql, 97)?;

assert_eq!(sql.as_command(), "SELECT * FROM user WHERE id = $1");
# Ok(())
# }
```

## Raw query

TOOD: show raw query usage

## No std

TODO: add `alloc` feature to opt-in the alloc crate

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

```rust
# use core::convert::Infallible;
use sqlstr::{WriteSql, SqlCommand, SqlExpr, Void, sqlexpr, sqlvalue};
use sqlstr::expr::{
	update_table, set_update, set_column, separator, item_separator, filter_where,
	lhs_binary_rhs, Cmp, math::MathBi
};

# fn main() -> Result<(), Infallible> {
let mut sql: SqlCommand<Void> = SqlCommand::default();

update_table(&mut sql, "product");
set_update(&mut sql);

set_column(&mut sql, "price");
lhs_binary_rhs(
	&mut sql,
	SqlExpr::<&str>::Expr("price"),
	MathBi::Add,
	SqlExpr::Value(50)
)?;
item_separator(&mut sql);

set_column(&mut sql, "discount");
separator(&mut sql);
sql.push_value(None as Option<f64>);
item_separator(&mut sql);

set_column(&mut sql, "image");
separator(&mut sql);
sql.push_cmd("DEFAULT");

filter_where(&mut sql);
lhs_binary_rhs(
	&mut sql,
	SqlExpr::<&str>::Expr("sku"),
	Cmp::Eq,
	SqlExpr::Value(32_u64),
)?;

assert_eq!(sql.as_command(), "UPDATE product SET price = price + $1, discount = $2, image = DEFAULT WHERE sku = $3");
# Ok(())
# }
```

## DELETE

```rust
# use core::convert::Infallible;
use sqlstr::{WriteSql, SqlCommand, SqlExpr, Void, sqlexpr, sqlvalue};
use sqlstr::expr::{delete_from, filter_where, lhs_binary_rhs, continue_condition, Cmp, Group, LogicBi};

# fn main() -> Result<(), Infallible> {
let mut sql: SqlCommand<Void> = SqlCommand::default();

delete_from(&mut sql, "product");

filter_where(&mut sql);

let mut group = Group::open(&mut sql);
lhs_binary_rhs(
	&mut group,
	SqlExpr::<&str>::Expr("price"),
	Cmp::Lte,
	SqlExpr::Value(100.0),
)?;
continue_condition(&mut group, LogicBi::And);
lhs_binary_rhs(
	&mut group,
	SqlExpr::<&str>::Expr("price"),
	Cmp::Gt,
	SqlExpr::Value(80.0),
)?;
group.close();

continue_condition(&mut sql, LogicBi::Or);

lhs_binary_rhs(
	&mut sql,
	SqlExpr::<&str>::Expr("ratings"),
	Cmp::Lt,
	SqlExpr::Value(0.2),
)?;

assert_eq!(sql.as_command(), "DELETE FROM product WHERE (price <= $1 AND price > $2) OR ratings < $3");
# Ok(())
# }
```

# Feature flags

- `std`: Enables the standard library features (mostly `Error` and `Display` impls)
- `fmt`: Enables the format features from the standard library (`Display` impls)
