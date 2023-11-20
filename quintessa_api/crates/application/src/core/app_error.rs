use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tokio::task::JoinError;

pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorResponse {
                error: self.message.clone(),
            }),
        )
            .into_response()
    }
}

//  to map from sea_orm::DbErr to AppError
impl From<sea_orm::DbErr> for AppError {
    fn from(value: sea_orm::DbErr) -> Self {
        eprintln!("Sea-orm database actions error: {}", value);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Sea-orm database actions error: {}", value),
        )
    }
}

// to map from chromiumoxide's String as Error to AppError
impl From<String> for AppError {
    fn from(value: String) -> Self {
        eprintln!("Chromiumoxide error String: {}", value);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Chromiumoxide error String: {}", value),
        )
    }
}

// to map from chromiumoxide's JoinError to AppError
impl From<JoinError> for AppError {
    fn from(value: JoinError) -> Self {
        eprintln!("Chromiumoxide JoinError: {}", value);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Chromiumoxide JoinError: {}", value),
        )
    }
}

// to map from reqwest error to AppError
impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        eprintln!("Reqwest error: {}", value);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Reqwest error: {}", value),
        )
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}
