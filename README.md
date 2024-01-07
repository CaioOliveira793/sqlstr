# sqlstr

A SQL (sqlstr) query builder.

## Features

- `#![no_std]` and no [`core::fmt`](https://doc.rust-lang.org/core/fmt/index.html)

## TODO

- [ ] Feature complete
  - [ ] [PostgreSQL commands](https://www.postgresql.org/docs/current/sql-commands.html)
  - [ ] [PostgreSQL functions](https://www.postgresql.org/docs/current/functions.html)
    - [ ] comparison
    - [ ] mathematical
    - [ ] pattern mathing
    - [ ] datetime
    - [ ] text search
    - [ ] array
    - [ ] aggregate
    - [ ] subquery
  - [ ] `fn count_column(column: &str)` -> `COUNT(<column>)`
  - [ ] `fn count_column_as(column: &str, alias: &str)` -> `COUNT(<column>) AS <alias>`
  - [ ] `fn uninon()` -> `UNION`
  - [ ] `fn uninon_all()` -> `UNION ALL`
  - [ ] `fn cte_with(name: &str)` -> `WITH <name> AS ()`
  - [ ] macro utils
    - [ ] `quoted!("public"."users" AS "u")` -> `'public'.'users' AS 'u'`
    - [ ] `doublequoted!("public"."users" AS "u")` -> `"public"."users" AS "u"`
    - [ ] `backquoted!("public"."users" AS "u")` -> "`public`.`users` AS `u`"
- [ ] Support more database dialects (other than Postgres)
  - [ ] Postgres
  - [ ] MySQL
  - [ ] SQLite
