use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod pages;

#[tokio::main]
async fn main() {
    // 1. Inicjalizacja logowania (zobaczysz w terminalu kto i gdzie wchodzi)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "portfolio=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Składanie modularnego routera
let app = Router::new()
    // Strona główna nie dodaje prefiksu, więc ją MERGE-ujemy
    .merge(pages::home::home::router())
    
    // Podstrony dodają prefiks /me oraz /projects, więc je NEST-ujemy
    .nest("/me", pages::me::me::router())
    .nest("/projects", pages::projects::projects::router());

    // 3. Uruchomienie serwera na porcie 8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("Listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}