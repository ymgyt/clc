#![warn(missing_docs)]

//! Clcgql is a GraphQL server for clc.

mod cache;
mod cli;
mod config;
mod dependency;
mod errors;
mod gql;
mod server;
mod signal;

fn init_logger() {
    use std::env::{set_var, var};
    use tracing_subscriber::{filter, fmt::time};

    if var(config::env::LOG_DIRECTIVES).is_err() {
        set_var(config::env::LOG_DIRECTIVES, "info")
    }
    if var(config::env::LOG_COLOR).is_err() {
        set_var(config::env::LOG_COLOR, "true")
    }

    tracing_subscriber::fmt()
        .with_timer(time::UtcTime::rfc_3339())
        .with_ansi(config::env::enable_log_color())
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_env_filter(filter::EnvFilter::from_env(config::env::LOG_DIRECTIVES))
        .init();
}

#[tokio::main]
async fn main() {
    init_logger();

    let signal = signal::receive();

    let app = cli::ClcGqlApp::parse();
    if let Err(err) = app.exec(signal).await {
        tracing::error!("{err}");
    }
}
