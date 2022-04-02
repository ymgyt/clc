use std::future::Future;
use std::io;
use std::net::{IpAddr, SocketAddr, TcpListener};

use crate::dependency::Dependency;
use crate::server::{Server, ServerConfig};
use clap::Parser;

use crate::signal::SignalEvent;

#[derive(Parser, Debug)]
#[clap(version, propagate_version = true, name = "clcgql")]
pub struct ClcGqlApp {
    /// Server listen ip.
    #[clap(long, default_value = "0.0.0.0")]
    pub ip: IpAddr,

    /// Server listen port.
    #[clap(long, default_value = "9696")]
    pub port: u16,
}

impl ClcGqlApp {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }

    /// Entry point of execution.
    pub async fn exec(self, shutdown: impl Future<Output = SignalEvent>) -> io::Result<()> {
        let dep = Dependency::new().await;
        self.listen_and_serve(dep, shutdown).await
    }

    async fn listen_and_serve(
        self,
        dep: Dependency,
        shutdown: impl Future<Output = SignalEvent>,
    ) -> io::Result<()> {
        let addr = SocketAddr::from((self.ip, self.port));

        tracing::info!(?addr, "Listening...");
        let listener = TcpListener::bind(addr)?;

        let server = Server::new(ServerConfig::default());

        if let Err(err) = server.serve(listener, dep, shutdown).await {
            tracing::error!("{err}");
        }
        Ok(())
    }
}
