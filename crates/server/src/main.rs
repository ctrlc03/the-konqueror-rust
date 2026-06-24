pub mod api;
pub mod app;
pub mod error;
pub mod state;

use clap::Parser;
use dashmap::DashMap;
use konqueror_common::protocol::WsMessage;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use crate::app::create_router;
use crate::state::AppState;
use konqueror_storage::memory::InMemoryStorage;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    address: String,
    #[arg(short, long)]
    port: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("konqueror=debug".parse().unwrap()),
        )
        .init();

    let cli = Cli::parse();

    info!(
        "Started The Konqueror with addr: {} and port: {}",
        cli.address, cli.port
    );

    let listener_channels: DashMap<Uuid, mpsc::Sender<WsMessage>> = DashMap::new();
    let client_channels: DashMap<Uuid, broadcast::Sender<WsMessage>> = DashMap::new();
    let in_memory_storage = InMemoryStorage::new();
    let storage = Arc::new(in_memory_storage);

    let app_state = Arc::new(AppState {
        storage,
        listener_channels,
        client_channels,
    });

    // build our application with a single route
    let app = create_router(app_state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &cli.address, &cli.port))
        .await
        .unwrap();

    info!("Listening on {}:{}", &cli.address, &cli.port);

    axum::serve(listener, app).await.unwrap();
}
