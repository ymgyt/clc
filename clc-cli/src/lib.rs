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

pub mod cli;
pub mod prompt;
pub mod repl;

pub use cli::ClcApp;
