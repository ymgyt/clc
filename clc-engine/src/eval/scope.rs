use crate::expression::Variable;
use std::collections::HashMap;

pub(super) struct Scope<'a> {
    binds: HashMap<&'a Variable, f64>,
}

impl<'a> Scope<'a> {
    pub(super) fn new() -> Self {
        Self {
            binds: HashMap::new(),
        }
    }

    pub(super) fn bind(&mut self, var: &'a Variable, n: f64) {
        self.binds.insert(var, n);
    }

    pub(super) fn resolve(&self, var: &Variable) -> Option<f64> {
        self.binds.get(var).copied()
    }
}
