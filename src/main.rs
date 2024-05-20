use async_graphql::{http::GraphiQLSource, Schema,EmptyMutation, EmptySubscription};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse}, routing::get, Router
};
use std::error::Error;

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // Query {
    //    hello(name: foo)
    //}
    async fn hello(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }
}

async fn graphql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphql_handler).post_service(GraphQL::new(schema)))
        .route("/version", get(version));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())    
}

async fn version() -> impl IntoResponse {
    env!("CARGO_PKG_VERSION")
}