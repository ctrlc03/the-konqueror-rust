use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use konqueror_common::{
    error::KonquerorError,
    protocol::WsMessage::{ImplantFirstCheckIn, ImplantResult},
    types::{ImplantCheckin, TaskResult},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::ApiError, types::ListenerState};

#[derive(Deserialize)]
pub struct GetTask {
    implant_id: Uuid,
}

#[derive(Deserialize)]
pub struct ImplantResultBody {
    task_id: Uuid,
    result: TaskResult,
}

// get tasks
pub async fn handle_tasks(
    State(state): State<Arc<ListenerState>>,
    Json(get_task): Json<GetTask>,
) -> Result<impl IntoResponse, ApiError> {
    let task = state.tasks.remove(&get_task.implant_id);

    match task {
        Some((_, t)) => Ok(Json(t)),
        None => Ok(Json(vec![])),
    }
}

// first checkin
pub async fn handle_checkin(
    State(state): State<Arc<ListenerState>>,
    Json(implant_checkin): Json<ImplantCheckin>,
) -> Result<impl IntoResponse, ApiError> {
    // call the server via websocket to get the task
    state
        .server_tx
        .send(ImplantFirstCheckIn(implant_checkin))
        .await
        .map_err(|e| KonquerorError::Transport(e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

// post result
pub async fn handle_result(
    State(state): State<Arc<ListenerState>>,
    Json(implant_result): Json<ImplantResultBody>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .server_tx
        .send(ImplantResult {
            task_id: implant_result.task_id,
            result: implant_result.result,
        })
        .await
        .map_err(|e| KonquerorError::Transport(e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
