use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Wrong password provided")]
    WrongPassword,

    #[error("User already exists")]
    UserExists,

    #[error("Token has expired")]
    ExpiredToken,

    #[error("Invalid authentication token")]
    InvalidToken,

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Not found: {0}")]
    NotFound(String),
}

impl IntoResponse for EngineError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            EngineError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "A database error occurred".to_string())
            }
            EngineError::WrongPassword => (StatusCode::UNAUTHORIZED, "Wrong password provided".to_string()),
            EngineError::UserExists => (StatusCode::CONFLICT, "User already exists".to_string()),
            EngineError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token has expired".to_string()),
            EngineError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authentication token".to_string()),
            EngineError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
            EngineError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            EngineError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            EngineError::Internal(ref e) => {
                tracing::error!("Internal error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred".to_string())
            }
            EngineError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
