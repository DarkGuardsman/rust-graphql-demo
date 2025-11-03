mod resolvers;
mod schema;
mod loaders;
mod datasource;
mod mem_tracing;


use anyhow::Context;
use async_graphql::{
    EmptyMutation, EmptySubscription, Error, Result, Schema, http::GraphiQLSource, ServerError,
};
use async_graphql::dataloader::DataLoader;
use async_graphql::extensions::Tracing;
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
use crate::mem_tracing::MemoryMetricsExtension;
use crate::schema::{Query, RoomLoader};

//#region [ENV Defaults]
const DEFAULT_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";
const DEFAULT_GRAPHQL_ROUTE: &str = "graphql";
//#endregion [ENV Defaults]

use alloc_metrics::MetricAlloc;

#[global_allocator]
static GLOBAL: MetricAlloc<std::alloc::System> = MetricAlloc::new(std::alloc::System);

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

    // use std::fs::File;
    // use std::sync::Arc;
    // let file = File::create(format!("debug-{}.log", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()))?;
    //
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_writer(Arc::new(file))
    //     .init();

    let server_address =
        std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| DEFAULT_ADDRESS.to_string());
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
    let graphql_path =
        std::env::var("ROUTE_GRAPHQL").unwrap_or_else(|_| DEFAULT_GRAPHQL_ROUTE.to_string());

    let use_room_loader = std::env::var("ENABLE_ROOM_LOADER").unwrap_or_else(|_| "false".to_string()) == "true";

    // Setup GraphQL
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .extension(MemoryMetricsExtension)
        .extension(Tracing)
        .data(AppContext::new(use_room_loader))
        .finish();

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

pub struct AppContext {
    pub use_room_loader: bool
}

impl AppContext {
    pub fn new(use_room_loader: bool) -> Self {
        Self {
            use_room_loader
        }
    }
}

#[derive(Clone)]
pub struct Headers {
    pub token: String,
}

pub struct DataLoaders {
    pub room_loader: DataLoader<RoomLoader>,
}

fn get_headers_for_context(headers: &HeaderMap) -> Result<Headers> {
    // TODO migrate to axum extractor
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

    let headers_result = get_headers_for_context(&headers);
    if let Err(error) = headers_result {
        // TODO revisit this as its a mess
        let server_error = ServerError::new(error.message.clone(), None);
        return GraphQLResponse::from(async_graphql::Response::from_errors(vec![server_error]));
    }

    let headers = headers_result.unwrap();
    let data_loaders = DataLoaders {
        // TODO see if there is a better way to pass token then to remake loader per request
        room_loader: DataLoader::new(RoomLoader { token: headers.token.clone() }, tokio::spawn),
    };

    req = req.data(headers).data(data_loaders);
    schema.execute(req).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    let graphql_path =
        std::env::var("ROUTE_GRAPHQL").unwrap_or_else(|_| DEFAULT_GRAPHQL_ROUTE.to_string());
    let path = format!("/{}", graphql_path);
    response::Html(GraphiQLSource::build().endpoint(&path).finish())
}
