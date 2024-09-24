use std::sync::Arc;

use axum::Router;
use tokio::sync::Mutex;

use tracing::info;

mod services {
    pub mod handlers;
    pub mod payloads;
}

#[derive(Clone)]
pub struct AppState {
    notes: Arc<Mutex<Vec<String>>>,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv()?;

    // TODO: enable logging here

    // TODO: read host and port from .env file
    let port = "PORT_FROM_ENV_FILE";
    let host = "HOST_FROM_EMV_FILE";

    let router = build_router();

    // Setup port
    // TODO: combine host and port into one string
    let listener = tokio::net::TcpListener::bind("HOST:PORT").await?;
    info!("Server is listening on port {}", port);
    // Start the server
    axum::serve(listener, router).await?;

    Ok(())
}

fn build_router() -> Router {
    let state = AppState {
        notes: Arc::new(Mutex::new(vec![])),
    };
    Router::<AppState>::new()
        // TODO: put routes here
        .with_state(state)
}
