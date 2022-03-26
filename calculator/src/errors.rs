use std::borrow::Cow;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInput(String),
    Eval(EvalError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidInput(msg) => write!(f, "invalid input: {}", msg),
            Eval(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub(crate) struct InvalidOperatorError<'a> {
    raw: Cow<'a, str>,
}

impl<'a> InvalidOperatorError<'a> {
    pub(crate) fn new(op: impl Into<Cow<'a, str>>) -> Self {
        InvalidOperatorError { raw: op.into() }
    }
}

impl<'a> fmt::Display for InvalidOperatorError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid operator '{}' currently ['+', '-', '*', '/'] supported",
            self.raw
        )
    }
}

impl<'a> std::error::Error for InvalidOperatorError<'a> {}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    ZeroDivision,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use EvalError::*;
        match self {
            ZeroDivision => write!(f, "divided by zero"),
        }
    }
}

impl std::error::Error for EvalError {}
