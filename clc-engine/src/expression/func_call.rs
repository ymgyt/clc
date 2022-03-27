use crate::expression::Expression;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FuncCall {
    ident: String,
    args: Vec<Expression>,
}

impl FuncCall {
    pub(crate) fn new(
        ident: impl Into<String>,
        args: impl IntoIterator<Item = impl Into<Expression>>,
    ) -> Self {
        Self {
            ident: ident.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }

    pub(crate) fn ident(&self) -> &str {
        self.ident.as_str()
    }

    pub(crate) fn args(&self) -> impl Iterator<Item = &Expression> {
        self.args.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_call_new() {
        let _ = FuncCall::new("sqrt", [1, 2, 3]);
        let _ = FuncCall::new("sqrt", vec![1, 2, 3]);
    }
}
