use crate::errors::EvalError;

pub(super) type FuncSignature = fn(&[f64]) -> Result<f64, EvalError>;

pub(super) fn resolve_func(ident: &str) -> Option<FuncSignature> {
    let f = match ident {
        builtin::SQRT => sqrt,
        builtin::POW => pow,
        builtin::ABS => abs,
        _ => return None,
    };
    Some(f)
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

pub(super) mod builtin {
    pub const SQRT: &str = "sqrt";
    pub const POW: &str = "pow";
    pub const ABS: &str = "abs";
}
