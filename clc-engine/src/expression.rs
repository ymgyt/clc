pub(crate) use func_call::FuncCall;

use std::fmt;

use crate::errors::InvalidOperatorError;

pub mod func_call;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expression {
    Literal(f64),
    FuncCall(FuncCall),
    Ast(Box<Node>),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Node {
    pub(crate) left: Expression,
    pub(crate) right: Expression,
    pub(crate) operator: Operator,
}

/// Calculation operator.
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum Operator {
    /// '+'
    Add,
    /// '-'
    Sub,
    /// '*'
    Mul,
    /// '/'
    Div,
}

impl<T> From<T> for Expression
where
    T: num_traits::AsPrimitive<f64>,
{
    fn from(n: T) -> Self {
        Expression::Literal(n.as_())
    }
}

impl From<Node> for Expression {
    fn from(node: Node) -> Self {
        Expression::ast(node)
    }
}

impl From<FuncCall> for Expression {
    fn from(fc: FuncCall) -> Self {
        Expression::func_call(fc)
    }
}

impl Expression {
    pub(crate) fn ast(node: Node) -> Self {
        Expression::Ast(Box::new(node))
    }

    pub(crate) fn func_call(fc: FuncCall) -> Self {
        Expression::FuncCall(fc)
    }
}

impl Node {
    pub(crate) fn new(left: Expression, operator: Operator, right: Expression) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Operator::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
        }
    }
}

impl<'a> TryFrom<&'a str> for Operator {
    type Error = InvalidOperatorError<'a>;

    fn try_from(op: &'a str) -> Result<Self, Self::Error> {
        use Operator::*;
        let op = match op {
            "+" => Add,
            "-" => Sub,
            "*" => Mul,
            "/" => Div,
            raw => return Err(InvalidOperatorError::new(raw)),
        };
        Ok(op)
    }
}

impl TryFrom<char> for Operator {
    type Error = InvalidOperatorError<'static>;

    fn try_from(op: char) -> Result<Self, Self::Error> {
        use Operator::*;
        let op = match op {
            '+' => Add,
            '-' => Sub,
            '*' => Mul,
            '/' => Div,
            raw => return Err(InvalidOperatorError::new(raw.to_string())),
        };
        Ok(op)
    }
}
