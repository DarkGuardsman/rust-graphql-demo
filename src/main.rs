mod schema;

use anyhow::Context;
use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{self, IntoResponse},
    routing::get,
};
use axum::serve::Listener;
use tokio::net::TcpListener;
use dotenv::dotenv;
use log::debug;
use crate::schema::Query;

async fn graphql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

//#region [ENV Defaults]
const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "3000";
const DEFAULT_GRAPHQL_ROUTE: &str = "graphql";
//#endregion [ENV Defaults]


// Entry point
fn main() -> anyhow::Result<()> {
    // Load ENVs from the .env file, ignore if it doesn't exist
    dotenv().ok();

    env_logger::init();

    // Start server
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(start_server())
}

// Async context for server
async fn start_server() -> anyhow::Result<()> {
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| { DEFAULT_ADDRESS.to_string()});
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| { DEFAULT_PORT.to_string()});
    let graphql_path = std::env::var("ROUTE_GRAPHQL").unwrap_or_else(|_| { DEFAULT_GRAPHQL_ROUTE.to_string()});

    // Setup GraphQL
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .finish();

    // Setup HTTP server
    let router = Router::new()
        .route(&format!("/{}", &graphql_path), get(graphql).post_service(GraphQL::new(schema)));

    let address = format!("{}:{}", server_address, server_port);
    let listener = TcpListener::bind(address).await.context("Failed to bind to address")?;

    #[cfg(feature = "local")] {
        let local_addr = listener.local_addr()?;
        debug!("Graph running: http://{}/{}", local_addr, graphql_path);
    }


    axum::serve(listener, router).await?;
    Ok(())
}
