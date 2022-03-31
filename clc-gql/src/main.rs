#![warn(missing_docs)]

//! Clcgql is a GraphQL server for clc.

mod cli;
mod config;
mod dependency;
mod gql;
mod server;
mod signal;

fn init_logger() {
    use tracing_subscriber::{filter, fmt::time};

    tracing_subscriber::fmt()
        .with_timer(time::UtcTime::rfc_3339())
        .with_ansi(true)
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
