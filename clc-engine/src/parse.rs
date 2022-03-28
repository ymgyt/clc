use crate::expression::Expression;
use crate::Error;

mod parse_impl;

pub(crate) struct Parser {}

impl Parser {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn parse_line(&self, input: &str) -> Result<Expression, Error> {
        parse_impl::parse_line(input)
            .map(|(remain, exp)| {
                if !remain.is_empty() {
                    eprintln!("unprocessed: {remain}");
                }
                exp
            })
            .map_err(|err| Error::InvalidInput(format!("{}", err)))
    }
}

impl Default for Parser {
    fn default() -> Self {
        Parser::new()
    }
}
