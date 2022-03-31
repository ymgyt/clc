mod handler;

use std::future::Future;
use std::net::TcpListener;

use axum::routing::{get, post};
use axum::Router;
use tower_http::trace::TraceLayer;

use crate::dependency::Dependency;
use crate::gql::schema;
use crate::signal::SignalEvent;

#[derive(Default)]
pub struct ServerConfig {}

pub struct Server {
    _config: ServerConfig,
}

impl Server {
    pub fn new(_config: ServerConfig) -> Self {
        Self { _config }
    }

    pub async fn serve(
        self,
        listener: TcpListener,
        Dependency { calculator }: Dependency,
        shutdown: impl Future<Output = SignalEvent>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let shutdown = async move {
            let sig = shutdown.await;
            tracing::info!(?sig, "Signal received");
        };

        let schema = schema::build().data(calculator).finish();

        let app = Router::new()
            .route("/health", get(handler::health_check))
            .route("/graphql", post(handler::graphql))
            .route("/graphql/playground", get(handler::graphql_playground))
            .layer(
                tower::ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http().on_request(
                        |req: &axum::http::Request<_>, _: &tracing::Span| {
                            tracing::info!(?req);
                        },
                    ))
                    .layer(axum::extract::Extension(schema)),
            );

        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown)
            .await?;

        tracing::info!("Server shutdown");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn server() {
        let server = Server::new(ServerConfig::default());
        let listener = std::net::TcpListener::bind(("127.0.0.1", 9696)).unwrap();
        let dep = Dependency::default();
        let (tx, rx) = tokio::sync::oneshot::channel();
        let shutdown = async move {
            rx.await.unwrap();
            SignalEvent::Interrupted
        };

        let handle = tokio::spawn(server.serve(listener, dep, shutdown));

        assert_eq!(
            reqwest::get("http://localhost:9696/health")
                .await
                .unwrap()
                .text()
                .await
                .unwrap(),
            "OK".to_owned(),
        );

        let body = reqwest::Client::new()
            .post("http://localhost:9696/graphql")
            .body(r#"{"operationName":null, "variables":{},"query": "{ eval(expression: \"100 - 1\") }"}"#)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert_eq!(body.as_str(), r#"{"data":{"eval":99.0}}"#);

        tx.send(()).unwrap();

        handle.await.unwrap().unwrap();
    }
}
