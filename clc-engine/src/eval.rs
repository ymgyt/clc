use crate::errors::EvalError;
use crate::parse::{Expression, Node, Operator};
use crate::Error;

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
        Expression::FuncCall(_fc) => unimplemented!(),
        Expression::Ast(node) => eval_node(node.as_ref()),
    }
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
                return Err(EvalError::ZeroDivision);
            }
            left / right
        }
    };
    Ok(eval)
}
