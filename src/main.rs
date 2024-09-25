use std::sync::Arc;
use tokio::sync::Mutex;

// Axum imports
use axum::{
    routing::{delete, get, post},
    Router,
};

// Logging
use tracing::info;

// Declare our folder structure in order to import it
mod services {
    pub mod handlers;
    pub mod payloads;
}
use crate::services::handlers::{create, delete_one, get_all, get_one};

// Our state is declared here
// We need to derive from Debug to be able to log it
// Clone is needed to be used with axum's router creation
#[derive(Clone, Debug)]
pub struct AppState {
    notes: Arc<Mutex<Vec<String>>>,
}

// tokio::main to enable the main function to be async
#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load environment
    dotenvy::dotenv()?;

    // TODO: enable logging here
    // Set some config for the subscriber
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_file(true)
        .with_line_number(true)
        .compact()
        .finish();

    // Register the subscriber, so that some logging actually happens
    tracing::subscriber::set_global_default(subscriber).ok();

    // TODO: read host and port from .env file
    let port = std::env::var("PORT").unwrap();
    let host = std::env::var("HOST").unwrap();

    // Router can be built in a function of course
    let router = build_router();

    // Setup port
    // TODO: combine host and port into one string
    // Here you can use the format! macro
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;
    info!("Server is listening on port {}", port);
    // Start the server
    axum::serve(listener, router).await?;

    Ok(())
}

fn build_router() -> Router {
    // Create our state
    let state = AppState {
        notes: Arc::new(Mutex::new(vec![])),
    };
    Router::<AppState>::new()
        // TODO: put routes here
        // You can of course have multiple routes with the same path as long as they use different
        // HTTP methods
        .route("/note/:index", get(get_one))
        .route("/notes", get(get_all))
        .route("/note", post(create))
        .route("/note", delete(delete_one))
        // Provide the state to axum
        .with_state(state)
}
