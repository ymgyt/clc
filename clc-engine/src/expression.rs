mod constant;
mod func_call;
mod lambda;
pub(crate) use constant::Constant;
pub(crate) use func_call::FuncCall;
pub(crate) use lambda::Lambda;

use std::fmt;

use crate::errors::InvalidOperatorError;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Expression {
    Literal(f64),
    Constant(Constant),
    Variable(Variable),
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
    /// '^'
    Pow,
}

impl<T> From<T> for Expression
where
    T: num_traits::AsPrimitive<f64>,
{
    fn from(n: T) -> Self {
        Expression::Literal(n.as_())
    }
}

impl From<Constant> for Expression {
    fn from(c: Constant) -> Self {
        Expression::constant(c)
    }
}

impl From<Variable> for Expression {
    fn from(v: Variable) -> Self {
        Expression::variable(v)
    }
}

impl From<FuncCall> for Expression {
    fn from(fc: FuncCall) -> Self {
        Expression::func_call(fc)
    }
}

impl From<Node> for Expression {
    fn from(node: Node) -> Self {
        Expression::ast(node)
    }
}

impl Expression {
    pub(crate) fn constant(c: Constant) -> Self {
        Expression::Constant(c)
    }

    pub(crate) fn variable(v: Variable) -> Self {
        Expression::Variable(v)
    }

    pub(crate) fn func_call(fc: FuncCall) -> Self {
        Expression::FuncCall(fc)
    }

    pub(crate) fn ast(node: Node) -> Self {
        Expression::Ast(Box::new(node))
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
            Pow => write!(f, "^"),
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
            "^" => Pow,
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
            '^' => Pow,
            raw => return Err(InvalidOperatorError::new(raw.to_string())),
        };
        Ok(op)
    }
}

/// Variable represents variable expression.
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) struct Variable {
    ident: String,
}

impl Variable {
    pub(crate) fn new(ident: impl Into<String>) -> Self {
        Self {
            ident: ident.into(),
        }
    }
}

impl From<Variable> for String {
    fn from(v: Variable) -> Self {
        v.ident
    }
}

impl AsRef<str> for Variable {
    fn as_ref(&self) -> &str {
        self.ident.as_str()
    }
}
