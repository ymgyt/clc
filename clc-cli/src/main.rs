#![warn(missing_docs)]

//! Clc is a Command line calculator written in Rust 🦀
//! It eval given expression and print result.

mod cli;
mod prompt;
mod repl;

fn main() {
    let app = cli::ClcApp::parse();
    if let Err(err) = app.exec() {
        eprintln!("{err}");
    }
}
