use std::borrow::Cow;
use std::fmt;
use std::fmt::Formatter;

/// Errors in calculation.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Provided input is invalid.
    InvalidInput(String),
    /// Error in eval stage.
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

#[derive(Debug)]
pub(crate) struct InvalidConstantError<'a> {
    raw: Cow<'a, str>,
}

impl<'a> InvalidConstantError<'a> {
    pub(crate) fn new(op: impl Into<Cow<'a, str>>) -> Self {
        InvalidConstantError { raw: op.into() }
    }
}

impl<'a> fmt::Display for InvalidConstantError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "invalid constant '{}'", self.raw)
    }
}

impl<'a> std::error::Error for InvalidConstantError<'a> {}

/// Error in eval stage.
#[derive(Debug, PartialEq)]
pub enum EvalError {
    /// zero division occurred.
    /// '100 / 0'
    DivisionByZero,
    /// undefined function called.
    /// 'foo(100)'
    UndefinedFunction {
        /// Undefined function identifier.
        ident: String,
    },
    /// Expected args not provided.
    /// 'pow(2)'
    ArgCountDoesNotMatch {
        /// Evaluated function identifier.
        ident: &'static str,
        /// Expected arg count.
        expected: usize,
        /// Actual arg count.
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
