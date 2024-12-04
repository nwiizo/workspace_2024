use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Invalid postal code format")]
    InvalidPostalCode,
    #[error("Address not found")]
    NotFound,
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::InvalidPostalCode => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::Internal(ref e) => {
                warn!("Internal server error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(serde_json::json!({
            "error": error_message,
            "status": status.as_u16(),
            "request_id": uuid::Uuid::new_v4().to_string()
        }));

        (status, body).into_response()
    }
}
