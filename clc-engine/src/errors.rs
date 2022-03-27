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
    /// zero division occurred.
    /// '100 / 0'
    DivisionByZero,
    /// undefined function called.
    /// 'foo(100)'
    UndefinedFunction { ident: String },
    /// Expected args not provided.
    /// 'pow(2)'
    ArgCountDoesNotMatch {
        ident: &'static str,
        expected: usize,
        actual: usize,
    },
}

impl EvalError {
    pub(crate) fn arg_count_does_not_match(
        ident: &'static str,
        expected: usize,
        actual: usize,
    ) -> Self {
        EvalError::ArgCountDoesNotMatch {
            ident,
            expected,
            actual,
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use EvalError::*;
        match self {
            DivisionByZero => write!(f, "divided by zero"),
            UndefinedFunction { ident } => write!(f, "function '{ident}' undefined"),
            ArgCountDoesNotMatch {
                ident,
                expected,
                actual,
            } => write!(
                f,
                "function '{ident}' arg count does not match. {actual} vs {expected}."
            ),
        }
    }
}

impl std::error::Error for EvalError {}
