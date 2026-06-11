use konqueror_common::error::KonquerorError;

pub struct ApiError(pub KonquerorError);

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self.0 {
            KonquerorError::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            KonquerorError::Unauthorized(_) => axum::http::StatusCode::UNAUTHORIZED,
            KonquerorError::AlreadyExists(_) => axum::http::StatusCode::CONFLICT,
            KonquerorError::InvalidInput(_) => axum::http::StatusCode::BAD_REQUEST,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = serde_json::json!({ "error": self.0.to_string() });
        (status, axum::Json(body)).into_response()
    }
}

impl From<KonquerorError> for ApiError {
    fn from(err: KonquerorError) -> Self {
        ApiError(err)
    }
}
