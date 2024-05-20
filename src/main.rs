mod query_root;
mod mutation_root;

use async_graphql::{http::GraphiQLSource, Schema, EmptySubscription};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse}, routing::get, Router
};
use std::error::Error;

use crate::{mutation_root::Mutation, query_root::QueryRoot};

async fn graphql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = Schema::build(QueryRoot, Mutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphql_handler).post_service(GraphQL::new(schema)))
        .route("/version", get(version));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())    
}

async fn version() -> impl IntoResponse {
    env!("CARGO_PKG_VERSION")
}