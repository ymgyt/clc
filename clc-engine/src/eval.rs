mod func_call;
mod scope;
use func_call::resolve_func;
use scope::Scope;

use crate::errors::{Error, EvalError};
use crate::eval::func_call::resolve_lambda_func;
use crate::expression::{Constant, Expression, FuncCall, Node, Operator, Variable};

pub(crate) struct Eval {}

impl Eval {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn eval(&self, exp: &Expression) -> Result<f64, Error> {
        let mut scope = Scope::new();
        eval_exp(exp, &mut scope).map_err(Error::Eval)
    }
}

fn eval_exp<'s>(exp: &'s Expression, scope: &mut Scope<'s>) -> Result<f64, EvalError> {
    match exp {
        Expression::Literal(lit) => Ok(*lit),
        Expression::Constant(cst) => eval_constant(cst),
        Expression::Variable(var) => eval_variable(var, scope),
        Expression::FuncCall(fc) => eval_func_call(fc, scope),
        Expression::Ast(node) => eval_node(node.as_ref(), scope),
    }
}

fn eval_constant(cst: &Constant) -> Result<f64, EvalError> {
    use std::f64;
    let cst = match cst {
        Constant::Pi => f64::consts::PI,
        Constant::E => f64::consts::E,
    };
    Ok(cst)
}

fn eval_variable(var: &Variable, scope: &Scope) -> Result<f64, EvalError> {
    scope
        .resolve(var)
        .ok_or_else(|| EvalError::VariableNotFound {
            ident: var.as_ref().to_owned(),
        })
}

/// eval given function call.
///   * resolve identifier.
///   * eval each args.
///   * then call func implementation with args
fn eval_func_call<'s>(fc: &'s FuncCall, scope: &mut Scope<'s>) -> Result<f64, EvalError> {
    match fc.lambda() {
        Some(lambda) => {
            let f =
                resolve_lambda_func(fc.ident()).ok_or_else(|| EvalError::UndefinedFunction {
                    ident: fc.ident().to_owned(),
                })?;

            let args = fc
                .args()
                .map(|arg| eval_exp(arg, scope))
                .collect::<Result<Vec<_>, _>>()?;

            f(args.as_slice(), lambda, scope)
        }
        None => {
            let f = resolve_func(fc.ident()).ok_or_else(|| EvalError::UndefinedFunction {
                ident: fc.ident().to_owned(),
            })?;
            // Eval each arg.
            let args = fc
                .args()
                .map(|arg| eval_exp(arg, scope))
                .collect::<Result<Vec<_>, _>>()?;

            f(args.as_slice())
        }
    }
}

fn eval_node<'s>(node: &'s Node, scope: &mut Scope<'s>) -> Result<f64, EvalError> {
    let left = eval_exp(&node.left, scope)?;
    let right = eval_exp(&node.right, scope)?;

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
        Operator::Pow => left.powf(right),
    };
    Ok(eval)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::{fc, lambda, node, var};

    #[test]
    fn divided_by_zero() {
        assert_eq!(
            apply_operator(10., Operator::Div, 0.),
            Err(EvalError::DivisionByZero)
        )
    }
    #[test]
    fn test_eval_constant() {
        assert_eq!(eval_constant(&Constant::Pi), Ok(std::f64::consts::PI));
        assert_eq!(eval_constant(&Constant::E), Ok(std::f64::consts::E));
    }

    #[test]
    fn test_eval_func_call() {
        assert_eq!(
            eval_func_call(&fc!("sqrt", 100), &mut Scope::new()),
            Ok(10.)
        );
        assert_eq!(
            eval_func_call(&fc!("sqrt", 10., 20.), &mut Scope::new()),
            Err(EvalError::arg_count_does_not_match("sqrt", 1, 2))
        );

        assert_eq!(eval_func_call(&fc!("pow", 2, 3), &mut Scope::new()), Ok(8.));
        assert_eq!(
            eval_func_call(&fc!("pow", 2), &mut Scope::new()),
            Err(EvalError::arg_count_does_not_match("pow", 2, 1))
        );

        assert_eq!(eval_func_call(&fc!("abs", -3.), &mut Scope::new()), Ok(3.));
        assert_eq!(eval_func_call(&fc!("abs", 3.), &mut Scope::new()), Ok(3.));
        assert_eq!(
            eval_func_call(&fc!("abs", 2, 2), &mut Scope::new()),
            Err(EvalError::arg_count_does_not_match("abs", 1, 2))
        );
    }

    #[test]
    fn test_eval_func_call_sig() {
        assert_eq!(
            eval_func_call(
                &fc!(
                    "sig",
                    [1, 5],
                    lambda!([var!("x")], node!(var!("x"), '^', 2))
                ),
                &mut Scope::new()
            ),
            Ok(55.)
        );
        assert_eq!(
            eval_func_call(
                &fc!(
                    "sig",
                    [5, 1],
                    lambda!([var!("x")], node!(var!("x"), '^', 2))
                ),
                &mut Scope::new()
            ),
            Ok(55.)
        );
        assert_eq!(
            eval_func_call(
                &fc!(
                    "sig",
                    [1, 1],
                    lambda!([var!("x")], node!(var!("x"), '^', 2))
                ),
                &mut Scope::new()
            ),
            Ok(1.)
        );
    }
}
