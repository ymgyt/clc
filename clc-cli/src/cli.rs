use clap::Parser;

use crate::repl;
use clc_engine::Calculator;

#[derive(Parser, Debug)]
#[clap(version, propagate_version = true, name = "clc")]
pub struct ClcApp {
    /// Eval given expression.
    #[clap(long, short = 'e')]
    pub eval: Option<String>,
}

impl ClcApp {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }

    /// Entry point of execution.
    pub fn exec(self) -> std::io::Result<()> {
        let cal = Calculator::new();

        match self.eval {
            Some(ref exp) => self.eval(exp, &cal),
            None => repl::repl(&cal),
        }
    }

    fn eval(&self, exp: &str, calculator: &Calculator) -> std::io::Result<()> {
        match calculator.calculate_line(exp) {
            Ok(eval) => println!("{eval}"),
            Err(err) => eprintln!("{err}"),
        }
        Ok(())
    }
}
