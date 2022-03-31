use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;
use axum::response::{self, IntoResponse};

use crate::gql::schema::AppSchema;

pub(super) async fn health_check() -> &'static str {
    "OK"
}

pub(super) async fn graphql(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub(super) async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
