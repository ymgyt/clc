use clc_engine::Calculator;

/// Dependency contains application dependencies.
pub struct Dependency {
    pub calculator: Calculator,
}

impl Dependency {
    pub fn new() -> Self {
        Self {
            calculator: Calculator::default(),
        }
    }
}

impl Default for Dependency {
    fn default() -> Self {
        Dependency::new()
    }
}
