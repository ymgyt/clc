//! [`Calculator`] calculate given expression.
//! Core functionality for [clc](https://crates.io/crates/clc).
//!
//! # Examples
//! ```rust
//! use clc_engine::Calculator;
//!
//! let clc = Calculator::new();
//! let eval = clc.calculate_line("sqrt(sqrt(16)) * (4 + 2)");
//!
//! assert_eq!(eval, Ok(12.));
//! ```
//!
//! ## Division by zero
//! ```rust
//! use clc_engine::{Calculator, Error,EvalError};
//!
//! let clc = Calculator::new();
//! let err = clc.calculate_line("10 / 0");
//!
//! assert_eq!(err, Err(Error::Eval(EvalError::DivisionByZero)));
//! ```
//!
//! Under the hood clc-engine use [nom](https://crates.io/crates/nom) to parse expression

mod calculator;
mod errors;
mod eval;
mod expression;
#[cfg(test)]
mod macros;
mod parse;

pub use {
    crate::calculator::Calculator,
    errors::{Error, EvalError},
};
