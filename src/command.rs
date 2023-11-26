use alloc::string::String;

use crate::{format_num::format_u32_base10, macros::display_sql_command};

pub trait ArgumentBuffer<T> {
    type Error;

    /// Push a new argument into the buffer
    fn push(&mut self, value: T) -> Result<(), Self::Error>;

    /// Returns the number of arguments pushed into `Self`
    fn count(&self) -> u32;
}

pub trait WriteSql<Arg> {
    fn push_expr<T>(&mut self, operand: SqlExpr<'_, T>) -> Result<(), Arg::Error>
    where
        Arg: ArgumentBuffer<T>;

    fn push_value<T>(&mut self, value: T) -> Result<(), Arg::Error>
    where
        Arg: ArgumentBuffer<T>;

    fn push_cmd(&mut self, expr: &str);
}

#[cfg_attr(feature = "fmt", derive(Debug))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SqlExpr<'ex, T> {
    Value(T),
    Expr(&'ex str),
}

pub const fn sqlexpr<T>(expr: &str) -> SqlExpr<'_, T> {
    SqlExpr::Expr(expr)
}

pub const fn sqlvalue<T>(expr: T) -> SqlExpr<'static, T> {
    SqlExpr::Value(expr)
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SqlCommand<Arg> {
    pub command: String,
    pub arguments: Arg,
}

impl<Arg> SqlCommand<Arg> {
    pub const fn new(command: String, arguments: Arg) -> Self {
        Self { command, arguments }
    }

    pub fn push_expr<T>(&mut self, operand: SqlExpr<'_, T>) -> Result<(), Arg::Error>
    where
        Arg: ArgumentBuffer<T>,
    {
        match operand {
            SqlExpr::Value(val) => self.push_value(val)?,
            SqlExpr::Expr(expr) => self.command.push_str(expr),
        }
        Ok(())
    }

    pub fn push_value<T>(&mut self, value: T) -> Result<(), Arg::Error>
    where
        Arg: ArgumentBuffer<T>,
    {
        self.arguments.push(value)?;
        let mut buf = [0; 10];
        self.command.push('$');
        self.command
            .push_str(format_u32_base10(self.arguments.count(), &mut buf));
        Ok(())
    }

    pub fn push_cmd(&mut self, expr: &str) {
        self.command.push_str(expr);
    }

    pub fn as_command(&self) -> &str {
        self.command.as_str()
    }
}

impl<Arg> WriteSql<Arg> for SqlCommand<Arg> {
    fn push_expr<T>(&mut self, operand: SqlExpr<'_, T>) -> Result<(), Arg::Error>
    where
        Arg: ArgumentBuffer<T>,
    {
        SqlCommand::push_expr(self, operand)
    }

    fn push_value<T>(&mut self, val: T) -> Result<(), <Arg>::Error>
    where
        Arg: ArgumentBuffer<T>,
    {
        SqlCommand::push_value(self, val)
    }

    fn push_cmd(&mut self, expr: &str) {
        SqlCommand::push_cmd(self, expr)
    }
}

display_sql_command!(SqlCommand);
