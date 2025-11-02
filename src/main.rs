mod resolvers;
mod schema;

use anyhow::Context;
use async_graphql::{
    EmptyMutation, EmptySubscription, Error, Result, Schema, http::GraphiQLSource, ServerError,
};
use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;
use axum::{
    Router,
    response::{self, IntoResponse},
    routing::get,
};
use axum::{extract::State, http::HeaderMap};
use dotenv::dotenv;
use tokio::net::TcpListener;

use crate::schema::Query;

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
    let server_address =
        std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| DEFAULT_ADDRESS.to_string());
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
    let graphql_path =
        std::env::var("ROUTE_GRAPHQL").unwrap_or_else(|_| DEFAULT_GRAPHQL_ROUTE.to_string());

    // Setup GraphQL
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    // Setup HTTP server
    let router = Router::new()
        .route(
            &format!("/{}", &graphql_path),
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(schema);
    //.route(&format!("/{}", &graphql_path), get(graphql).post_service(GraphQL::new(schema)));

    let address = format!("{}:{}", server_address, server_port);
    let listener = TcpListener::bind(address)
        .await
        .context("Failed to bind to address")?;

    #[cfg(feature = "local")]
    {
        let local_addr = listener.local_addr()?;
        log::info!("Graph running: http://{}/{}", local_addr, graphql_path);
    }

    axum::serve(listener, router).await?;
    Ok(())
}

#[derive(Clone)]
pub struct Headers {
    pub token: String,
}

fn get_headers_for_context(headers: &HeaderMap) -> Result<Headers> {
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| s.to_string()).ok());

    if token.is_none() {
        return Err(Error::new("Missing Authorization header"));
    }
    return Ok(Headers { token: token.unwrap() });
}

async fn graphql_handler(
    headers: HeaderMap, // Extract headers
    State(schema): State<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    let headers = get_headers_for_context(&headers);
    if headers.is_err() {
        let error = headers.err().unwrap(); // TODO revisit this as its a mess
        let server_error = ServerError::new(error.message.clone(), None);
        return GraphQLResponse::from(async_graphql::Response::from_errors(vec![server_error]));
    }

    req = req.data(headers.unwrap());
    schema.execute(req).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    let graphql_path =
        std::env::var("ROUTE_GRAPHQL").unwrap_or_else(|_| DEFAULT_GRAPHQL_ROUTE.to_string());
    let path = format!("/{}", graphql_path);
    response::Html(GraphiQLSource::build().endpoint(&path).finish())
}
