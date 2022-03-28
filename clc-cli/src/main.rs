#![warn(missing_docs)]

//! Clc is a Command line calculator written in Rust 🦀
//!
//! # Usage
//! ```text
//! $ clc
//! Version: v0.1.2
//! To quit, press Ctrl+C or type quit
//! ❯ sqrt(sqrt(16)) * (100 - 1) * (100 + 1) / 9
//! 2222
//! ❯ quit
//! bye
//! ```

mod cli;
mod prompt;
mod repl;

fn main() {
    let app = cli::ClcApp::parse();
    if let Err(err) = app.exec() {
        eprintln!("{err}");
    }
}
