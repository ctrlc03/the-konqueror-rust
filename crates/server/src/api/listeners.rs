use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use konqueror_common::types::Listener;
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

pub async fn get_listeners(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let listeners = state.storage.list_active_listeners().await?;

    Ok(Json(listeners))
}

pub async fn get_listener(
    State(state): State<Arc<AppState>>,
    Path(listener_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let listener = state.storage.get_listener(listener_id).await?;

    Ok(Json(listener))
}

pub async fn delete_listener(
    State(state): State<Arc<AppState>>,
    Path(listener_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state.storage.delete_listener(listener_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_listener(
    State(state): State<Arc<AppState>>,
    Json(listener): Json<Listener>,
) -> Result<impl IntoResponse, ApiError> {
    state.storage.create_listener(&listener).await?;

    Ok(StatusCode::NO_CONTENT)
}
