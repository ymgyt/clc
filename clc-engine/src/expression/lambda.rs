use crate::expression::{Expression, Variable};

/// Lambda represents anonymous function. like '|x| { x^2 }'
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Lambda {
    arg: Variable,
    body: Expression,
}

impl Lambda {
    pub(crate) fn new(arg: Variable, body: Expression) -> Self {
        Self { arg, body }
    }

    pub(crate) fn arg(&self) -> &Variable {
        &self.arg
    }

    pub(crate) fn body(&self) -> &Expression {
        &self.body
    }
}
