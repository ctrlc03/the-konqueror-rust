use dashmap::DashMap;
use konqueror_common::{protocol::WsMessage, types::Task};
use tokio::sync::mpsc;
use uuid::Uuid;

pub struct ListenerState {
    pub tasks: DashMap<Uuid, Vec<Task>>,
    pub server_tx: mpsc::Sender<WsMessage>,
    pub listener_id: Uuid,
}
