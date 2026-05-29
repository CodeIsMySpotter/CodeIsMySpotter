use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod pages;

/// Main entry point for the portfolio application.
#[tokio::main]
async fn main() {
    // 1. Initialize logging (shows incoming requests and their paths in the terminal)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "portfolio=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Assemble the modular router
    let app = Router::new()
        // The home page does not add a prefix, so we MERGE it
        .merge(pages::home::home::router())
        
        // Subpages add the /me and /projects prefixes, so we NEST them
        .nest("/me", pages::me::me::router())
        .nest("/projects", pages::projects::projects::router());

    // 3. Start the server on port 8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("Listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}