mod func_call;

use crate::errors::{Error, EvalError};
use crate::expression::{Expression, FuncCall, Node, Operator};
use func_call::resolve_func;

pub(crate) struct Eval {}

impl Eval {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn eval(&self, exp: &Expression) -> Result<f64, Error> {
        eval_exp(exp).map_err(Error::Eval)
    }
}

fn eval_exp(exp: &Expression) -> Result<f64, EvalError> {
    match exp {
        Expression::Literal(lit) => Ok(*lit),
        Expression::FuncCall(fc) => eval_func_call(fc),
        Expression::Ast(node) => eval_node(node.as_ref()),
    }
}

/// eval given function call.
///   * resolve identifier.
///   * eval each args.
///   * then call func implementation with args
fn eval_func_call(fc: &FuncCall) -> Result<f64, EvalError> {
    let f = resolve_func(fc.ident()).ok_or_else(|| EvalError::UndefinedFunction {
        ident: fc.ident().to_owned(),
    })?;
    // Eval each arg.
    let args = fc.args().map(eval_exp).collect::<Result<Vec<_>, _>>()?;

    f(args.as_slice())
}

fn eval_node(node: &Node) -> Result<f64, EvalError> {
    let left = eval_exp(&node.left)?;
    let right = eval_exp(&node.right)?;

    apply_operator(left, node.operator, right)
}

fn apply_operator(left: f64, op: Operator, right: f64) -> Result<f64, EvalError> {
    let eval = match op {
        Operator::Add => left + right,
        Operator::Sub => left - right,
        Operator::Mul => left * right,
        Operator::Div => {
            if right == 0. {
                return Err(EvalError::DivisionByZero);
            }
            left / right
        }
    };
    Ok(eval)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::fc;

    #[test]
    fn divided_by_zero() {
        assert_eq!(
            apply_operator(10., Operator::Div, 0.),
            Err(EvalError::DivisionByZero)
        )
    }

    #[test]
    fn test_eval_func_call() {
        assert_eq!(eval_func_call(&fc!("sqrt", 100)), Ok(10.));
        assert_eq!(
            eval_func_call(&fc!("sqrt", 10., 20.)),
            Err(EvalError::arg_count_does_not_match("sqrt", 1, 2))
        );
    }
}
