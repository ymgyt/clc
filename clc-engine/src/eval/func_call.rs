use std::cmp::{max, min};

use crate::errors::EvalError;
use crate::eval::eval_exp;
use crate::eval::scope::Scope;
use crate::expression::Lambda;

pub(super) type FuncSignature = fn(&[f64]) -> Result<f64, EvalError>;
pub(super) type LambdaFuncSignature<'s> =
    fn(&[f64], &'s Lambda, &mut Scope<'s>) -> Result<f64, EvalError>;

pub(super) fn resolve_func(ident: &str) -> Option<FuncSignature> {
    let f = match ident {
        builtin::SQRT => sqrt,
        builtin::POW => pow,
        builtin::ABS => abs,
        _ => return None,
    };
    Some(f)
}

pub(super) fn resolve_lambda_func(ident: &str) -> Option<LambdaFuncSignature> {
    let f = match ident {
        builtin::SIG => sig,
        _ => return None,
    };
    Some(f)
}

pub(super) mod builtin {
    pub const SQRT: &str = "sqrt";
    pub const POW: &str = "pow";
    pub const ABS: &str = "abs";
    pub const SIG: &str = "sig";
}

fn sqrt(args: &[f64]) -> Result<f64, EvalError> {
    match args {
        [arg] => Ok(arg.sqrt()),
        _ => Err(EvalError::arg_count_does_not_match(
            builtin::SQRT,
            1,
            args.len(),
        )),
    }
}

fn pow(args: &[f64]) -> Result<f64, EvalError> {
    match args {
        [base, n] => Ok(base.powf(*n)),
        _ => Err(EvalError::arg_count_does_not_match(
            builtin::POW,
            2,
            args.len(),
        )),
    }
}

fn abs(args: &[f64]) -> Result<f64, EvalError> {
    match args {
        [n] => Ok(n.abs()),
        _ => Err(EvalError::arg_count_does_not_match(
            builtin::ABS,
            1,
            args.len(),
        )),
    }
}

fn sig<'s>(args: &[f64], lambda: &'s Lambda, scope: &mut Scope<'s>) -> Result<f64, EvalError> {
    let (&start, &end) = match args {
        [start, end] => (start, end),
        _ => {
            return Err(EvalError::arg_count_does_not_match(
                builtin::SIG,
                2,
                args.len(),
            ))
        }
    };
    // Should we return error ?
    let (start, end) = (start.max(0.) as usize, end.max(0.) as usize);
    // Calculate sig(1,3) and sig(3,1) as equal.
    let (start, end) = (min(start, end), max(start, end));

    (start..=end).into_iter().try_fold(0., |acc, idx| {
        scope.bind(lambda.arg(), idx as f64);
        eval_exp(lambda.body(), scope).map(|eval| acc + eval)
    })
}
