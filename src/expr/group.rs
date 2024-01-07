use core::marker::PhantomData;

use super::separator_optional;
use crate::WriteSql;

pub struct Group<'cmd, Sql: WriteSql<Arg>, Arg>(&'cmd mut Sql, PhantomData<Arg>);

impl<'cmd, Sql, Arg> Group<'cmd, Sql, Arg>
where
    Sql: WriteSql<Arg>,
{
    pub fn open(sql: &'cmd mut Sql) -> Self {
        separator_optional(sql);
        sql.push_cmd("(");
        Self(sql, PhantomData)
    }

    pub fn close(self) {
        core::mem::drop(self)
    }

    pub fn sql(&mut self) -> &mut Sql {
        self.0
    }
}

impl<'cmd, Sql, Arg> WriteSql<Arg> for Group<'cmd, Sql, Arg>
where
    Sql: WriteSql<Arg>,
{
    fn push_expr<T>(&mut self, operand: crate::SqlExpr<'_, T>) -> Result<(), <Arg>::Error>
    where
        Arg: crate::ArgumentBuffer<T>,
    {
        self.0.push_expr(operand)
    }

    fn push_value<T>(&mut self, value: T) -> Result<(), <Arg>::Error>
    where
        Arg: crate::ArgumentBuffer<T>,
    {
        self.0.push_value(value)
    }

    fn push_cmd(&mut self, expr: &str) {
        self.0.push_cmd(expr)
    }

    fn as_command(&self) -> &str {
        self.0.as_command()
    }
}

impl<'cmd, Sql, Arg> Drop for Group<'cmd, Sql, Arg>
where
    Sql: WriteSql<Arg>,
{
    fn drop(&mut self) {
        self.0.push_cmd(")");
    }
}

#[cfg(test)]
mod test {
    use alloc::string::String;

    use super::Group;
    use crate::{test::TestArgs, SqlCommand};

    #[test]
    fn group_condition() {
        let mut sql = SqlCommand::new(String::new(), TestArgs::new());
        let mut group = Group::open(&mut sql);
        group.sql().push_cmd("user.name = ");
        group.sql().push_value("Rust").unwrap();
        group.sql().push_cmd(" AND user.active = ");
        group.sql().push_value(true).unwrap();
        group.close();

        assert_eq!(sql.as_command(), "(user.name = $1 AND user.active = $2)");
        assert_eq!(sql.arguments.as_str(), "Rust;true;");
    }

    #[test]
    fn group_scope() {
        let mut sql: SqlCommand<TestArgs> = SqlCommand::default();
        {
            let mut group = Group::open(&mut sql);
            group.sql().push_cmd("user.name = ");
            group.sql().push_value("Rust").unwrap();
            group.sql().push_cmd(" AND user.active = ");
            group.sql().push_value(true).unwrap();
        }

        assert_eq!(sql.as_command(), "(user.name = $1 AND user.active = $2)");
        assert_eq!(sql.arguments.as_str(), "Rust;true;");
    }
}
