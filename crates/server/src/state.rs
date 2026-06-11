use dashmap::DashMap;
use konqueror_common::{protocol::WsMessage, storage::Storage};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

pub struct AppState {
    pub storage: Arc<dyn Storage>,
    pub listener_channels: DashMap<Uuid, mpsc::Sender<WsMessage>>,
    pub client_channels: DashMap<Uuid, broadcast::Sender<WsMessage>>,
}
