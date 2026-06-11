use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use konqueror_common::types::Implant;
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

pub async fn get_implants(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let implants = state.storage.list_active_implants().await?;

    Ok(Json(implants))
}

pub async fn get_implant_by_listener(
    State(state): State<Arc<AppState>>,
    Path(listener_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let implant = state.storage.get_implant_by_listener(listener_id).await?;

    Ok(Json(implant))
}

pub async fn delete_implant(
    State(state): State<Arc<AppState>>,
    Path(implant_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    state.storage.delete_implant(implant_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_implant(
    State(state): State<Arc<AppState>>,
    Json(implant): Json<Implant>,
) -> Result<impl IntoResponse, ApiError> {
    state.storage.create_implant(&implant).await?;

    Ok(StatusCode::NO_CONTENT)
}
