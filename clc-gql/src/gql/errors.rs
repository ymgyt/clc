use std::fmt::{self, Formatter};

use async_graphql::{Error, ErrorExtensions};
use clc_engine::{Error as ClcError, EvalError};

#[derive(Debug, PartialEq)]
pub enum GraphqlError {
    EvalError(EvalError),
    InvalidEvalExpression(String),
}

impl fmt::Display for GraphqlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use GraphqlError::*;
        match self {
            EvalError(err) => write!(f, "eval error: {err}"),
            InvalidEvalExpression(err) => write!(f, "invalid expression: {err}"),
        }
    }
}

impl std::error::Error for GraphqlError {}

impl ErrorExtensions for GraphqlError {
    fn extend(&self) -> Error {
        Error::new(format!("{self}")).extend_with(|_err, e| match self {
            GraphqlError::EvalError(_) => e.set("code", "EVAL_ERROR"),
            GraphqlError::InvalidEvalExpression(_) => e.set("code", "INVALID_EXPRESSION"),
        })
    }
}

impl From<ClcError> for GraphqlError {
    fn from(err: ClcError) -> Self {
        use clc_engine::Error::*;
        match err {
            Eval(err) => GraphqlError::EvalError(err),
            InvalidInput(err) => GraphqlError::InvalidEvalExpression(err),
        }
    }
}
