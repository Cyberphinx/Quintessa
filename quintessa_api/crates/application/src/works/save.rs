use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, TryIntoModel};

use domain::work::{self, Model as WorkModel};

use crate::core::app_error::AppError;

pub async fn save_active_work(
    db: &DatabaseConnection,
    work: work::ActiveModel,
) -> Result<WorkModel, AppError> {
    let work = work.save(db).await.map_err(|error| {
        eprintln!("Error saving work: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again",
        )
    })?;

    convert_active_to_model(work)
}

fn convert_active_to_model(active_work: work::ActiveModel) -> Result<WorkModel, AppError> {
    active_work.try_into_model().map_err(|error| {
        eprintln!("Error converting work's active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
