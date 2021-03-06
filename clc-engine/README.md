# clc-engine

[![clc-engine on crates.io](https://img.shields.io/crates/v/clc-engine)](https://crates.io/crates/clc-engine)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://docs.rs/clc-engine/)
[![Changelog](https://img.shields.io/badge/changelog-latest-blue)](https://github.com/ymgyt/clc/blob/main/CHANGELOG.md)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE-MIT)

[`Calculator`] calculate given expression.
Core functionality for [clc](https://crates.io/crates/clc).

## Examples
```rust
use clc_engine::Calculator;

let clc = Calculator::new();
let eval = clc.calculate_line("sqrt(sqrt(16)) * (4 + 2)");

assert_eq!(eval, Ok(12.));
```

### Division by zero
```rust
use clc_engine::{Calculator, Error,EvalError};

let clc = Calculator::new();
let err = clc.calculate_line("10 / 0");

assert_eq!(err, Err(Error::Eval(EvalError::DivisionByZero)));
```

Under the hood clc-engine use [nom](https://crates.io/crates/nom) to parse expression

## License

This project is available under the terms of either the [Apache 2.0 license](../LICENSE-APACHE) or the [MIT license](../LICENSE-MIT).
