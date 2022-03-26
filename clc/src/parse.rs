mod parse_impl;

use crate::Error;
use std::fmt;

use crate::errors::InvalidOperatorError;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expression {
    Literal(f64),
    #[allow(dead_code)]
    FuncCall(FuncCall),
    Ast(Box<Node>),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Node {
    pub(crate) left: Expression,
    pub(crate) right: Expression,
    pub(crate) operator: Operator,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FuncCall {
    ident: String,
    args: Vec<Expression>,
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

impl Expression {
    fn ast(node: Node) -> Self {
        Expression::Ast(Box::new(node))
    }
}

impl Node {
    fn new(left: Expression, operator: Operator, right: Expression) -> Self {
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

pub(crate) struct Parser {}

impl Parser {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn parse_line(&self, input: &str) -> Result<Expression, Error> {
        parse_impl::parse_line(input)
            .map(|(_, exp)| exp)
            .map_err(|err| Error::InvalidInput(format!("{}", err)))
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}
