use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Result;
use crate::types::{Implant, ImplantCheckin, Task, TaskResult};

/// Messages sent over WebSocket between server <-> client and server <-> listener.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
#[serde(rename_all = "snake_case")]
pub enum WsMessage {
    // --- Server -> Client ---
    ImplantCheckedIn(Implant),
    TaskCompleted(Task),

    // --- Server -> Listener ---
    NewTask(Task),
    KillListener,

    // --- Listener -> Server ---
    ImplantFirstCheckIn(ImplantCheckin),
    ImplantResult { task_id: Uuid, result: TaskResult },
    ImplantShutdown { implant_id: Uuid },

    // --- Bidirectional ---
    Ping,
    Pong,
    Error { message: String },
}

/// Encrypted message envelope used between implant <-> listener over HTTP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C2Envelope {
    pub message: String,
    pub hmac: String,
    #[serde(rename = "type")]
    pub kind: C2MessageKind,
    pub listener_id: Uuid,
    pub sequence: u64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum C2MessageKind {
    FirstCheckin,
    Result,
    ImplantShutdown,
    KeyExchange,
    KeyExchangeResponse,
}

pub fn encode_envelope(envelope: &C2Envelope) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(envelope)?)
}

pub fn decode_envelope(encoded: &[u8]) -> Result<C2Envelope> {
    Ok(serde_json::from_slice::<C2Envelope>(encoded)?)
}

pub fn encoded_ws_message(ws_message: &WsMessage) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(ws_message)?)
}

pub fn decode_ws_message(encoded: &[u8]) -> Result<WsMessage> {
    Ok(serde_json::from_slice::<WsMessage>(encoded)?)
}
