use clap::Parser;

use crate::repl;

#[derive(Parser, Debug)]
#[clap(version, propagate_version = true)]
pub struct ClcApp {
    /// Eval given expression.
    #[clap(long, short = 'E')]
    pub eval: Option<String>,
}

impl ClcApp {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }

    pub fn exec(self) -> std::io::Result<()> {
        repl::repl()
    }
}
