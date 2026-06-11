use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use konqueror_common::{
    crypto::{hash_password, verify_password},
    error::KonquerorError,
    types::Operator,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub api_key: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let operator = state
        .storage
        .get_operator_by_username(&body.username)
        .await?;

    // verify password
    let valid = verify_password(&body.password, &operator.password_hash)?;
    if !valid {
        return Err(ApiError::from(KonquerorError::Unauthorized(
            "invalid password".to_string(),
        )));
    }

    let api_key = operator
        .api_key
        .ok_or(KonquerorError::Unauthorized("no API key".to_string()))?;

    state
        .storage
        .set_operator_logged_in(operator.id, true)
        .await?;

    Ok(Json(LoginResponse { api_key }))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, ApiError> {
    let api_key = headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .ok_or(KonquerorError::Unauthorized("missing API key".to_string()))?;

    let operator = state.storage.get_operator_by_api_key(api_key).await?;

    state
        .storage
        .set_operator_logged_in(operator.id, false)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(LoginRequest { username, password }): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let password_hash = hash_password(&password)?;

    let id = Uuid::new_v4();
    let api_key = Uuid::new_v4();

    let operator = Operator {
        username,
        password_hash,
        id,
        is_admin: false,
        api_key: Some(api_key.to_string()),
        logged_in: false,
    };

    state.storage.create_operator(&operator).await?;

    Ok(StatusCode::CREATED)
}
