use crate::expression::{Expression, Lambda};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FuncCall {
    ident: String,
    args: Vec<Expression>,
    lambda: Option<Box<Lambda>>,
}

impl FuncCall {
    pub(crate) fn new(
        ident: impl Into<String>,
        args: impl IntoIterator<Item = impl Into<Expression>>,
        lambda: Option<Lambda>,
    ) -> Self {
        Self {
            ident: ident.into(),
            args: args.into_iter().map(Into::into).collect(),
            lambda: lambda.map(Box::new),
        }
    }

    pub(crate) fn ident(&self) -> &str {
        self.ident.as_str()
    }

    pub(crate) fn args(&self) -> impl Iterator<Item = &Expression> {
        self.args.iter()
    }

    pub(crate) fn lambda(&self) -> Option<&Lambda> {
        self.lambda.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_call_new() {
        let _ = FuncCall::new("sqrt", [1, 2, 3], None);
        let _ = FuncCall::new("sqrt", vec![1, 2, 3], None);
    }
}
