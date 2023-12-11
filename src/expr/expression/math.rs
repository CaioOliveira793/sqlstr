use super::{BinaryOperator, UnaryOperator, WriteSql};

/// Math binary operators
#[cfg_attr(any(feature = "fmt", test, debug_assertions), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MathBi {
    /// Adition `+`
    ///
    /// 2 + 3
    Add,
    /// Subtration `-`
    ///
    /// 2 - 3
    Sub,
    /// Multiplication `*`
    ///
    /// 5 * 2
    Mult,
    /// Division `/`
    ///
    /// 4 / 2
    Div,
    /// Module `%`
    ///
    /// 3 % 2
    Mod,
    /// Bitwise and `&`
    ///
    /// 0b0101 & 0b0001
    BitAnd,
    /// Bitwise or `|`
    ///
    /// 0b0101 | 0b0001
    BitOr,
    /// Bitwise xor `^`
    ///
    /// 0b0101 ^ 0b0001
    BitXor,
    /// Bit shift left `<<`
    ///
    /// 0b0101 << 0b0001
    ShiftLeft,
    /// Bit shift left `>>`
    ///
    /// 0b0101 >> 0b0001
    ShiftRight,
}

impl MathBi {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mult => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::ShiftLeft => "<<",
            Self::ShiftRight => ">>",
        }
    }
}

impl super::private::Sealed for MathBi {}

impl BinaryOperator for MathBi {
    fn push_operator<Sql, Arg>(&self, sql: &mut Sql)
    where
        Sql: WriteSql<Arg>,
    {
        sql.push_cmd(self.as_str())
    }
}

/// Math binary operators
#[cfg_attr(any(feature = "fmt", test, debug_assertions), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MathUnary {
    /// Negation `-`
    ///
    /// - (-1)
    Neg,
    /// Bitwise not `~`
    ///
    /// ~ 0b0101
    BitNot,
}

impl MathUnary {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Neg => "!",
            Self::BitNot => "~",
        }
    }
}

impl super::private::Sealed for MathUnary {}

impl UnaryOperator for MathUnary {
    fn push_operator<Sql, Arg>(&self, sql: &mut Sql)
    where
        Sql: WriteSql<Arg>,
    {
        sql.push_cmd(self.as_str())
    }
}

/// Math functions
#[cfg_attr(any(feature = "fmt", test, debug_assertions), derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MathFn {
    /// Square root
    Sqrt,
    /// Exponentiation `^`
    ///
    /// 2 ^ 8
    Exp,
    /// Factorial
    ///
    /// 5!
    Factorial,
    /// Absolute
    ///
    /// |-3|
    Abs,
}
