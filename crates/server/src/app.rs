use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    api::{
        auth::{login, logout, register},
        implants::{create_implant, get_implant_by_listener, get_implants},
        listeners::{create_listener, get_listener, get_listeners},
    },
    state::AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let app = Router::new()
        .route("/health", get(|| async { "Ok" }))
        .route("/api/implant/{id}", get(get_implant_by_listener))
        .route("/api/implants", get(get_implants))
        .route("/api/implants", post(create_implant))
        .route("/api/listeners", post(create_listener))
        .route("/api/listeners", get(get_listeners))
        .route("/api/listener/{id}", get(get_listener))
        .route("/api/login", post(login))
        .route("/api/logout", get(logout))
        .route("/api/register", post(register))
        .with_state(app_state);

    app
}
