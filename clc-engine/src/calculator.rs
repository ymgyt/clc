use crate::errors::Error;
use crate::eval::Eval;
use crate::parse::Parser;

/// Calculator provide calculation api.
pub struct Calculator {
    parser: Parser,
    eval: Eval,
}

impl Calculator {
    /// Construct Calculator.
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            eval: Eval::new(),
        }
    }

    /// Calculate single line.
    pub fn calculate_line(&self, input: &str) -> Result<f64, Error> {
        self.parser
            .parse_line(input)
            .and_then(|exp| self.eval.eval(&exp))
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::EvalError;

    #[test]
    fn calculate_line() {
        let c = calculator();

        assert_eq!(c.calculate_line("1 + 2"), Ok(3.));
        assert_eq!(c.calculate_line("-2 * -2"), Ok(4.));
        assert_eq!(c.calculate_line("sqrt(sqrt(16)) + 10"), Ok(12.));
        assert_eq!(c.calculate_line("sqrt(pow(-2, 2)) - abs(-2)"), Ok(0.));
        assert_eq!(c.calculate_line("pow(2,3) - 2 ^ 3"), Ok(0.));
    }

    #[test]
    fn divide_by_zero() {
        let c = calculator();

        assert_eq!(
            c.calculate_line("1 / 0"),
            Err(Error::Eval(EvalError::DivisionByZero))
        );
    }

    fn calculator() -> Calculator {
        Calculator::new()
    }
}
