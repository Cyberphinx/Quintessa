use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::utilities::app_error::AppError;

pub mod hello_world;
pub mod media_controller;
pub mod projects_controller;
pub mod users_controller;

pub async fn handle_result<T>(
    response: Result<Json<T>, application::core::app_error::AppError>,
) -> Result<Json<T>, AppError> {
    match response {
        Ok(result) => Ok(result),
        Err(error) => {
            eprintln!("{}", error.message);
            Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong",
            ))
        }
    }
}

pub async fn handle_paged_result<T>(
    response: Result<(HeaderMap, Json<T>), application::core::app_error::AppError>,
) -> Result<(HeaderMap, Json<T>), AppError> {
    match response {
        Ok(result) => Ok(result),
        Err(error) => {
            eprintln!("{}", error.message);
            Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong",
            ))
        }
    }
}

pub async fn handle_void_result(
    response: Result<(), application::core::app_error::AppError>,
) -> Result<(), AppError> {
    match response {
        Ok(result) => Ok(result),
        Err(error) => {
            eprintln!("{}", error.message);
            Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong",
            ))
        }
    }
}
