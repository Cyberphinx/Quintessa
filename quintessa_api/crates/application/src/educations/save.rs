use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, TryIntoModel};

use domain::education::{self, Model as EducationModel};

use crate::core::app_error::AppError;

pub async fn save_active_education(
    db: &DatabaseConnection,
    education: education::ActiveModel,
) -> Result<EducationModel, AppError> {
    let education = education.save(db).await.map_err(|error| {
        eprintln!("Error saving education: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again",
        )
    })?;

    convert_active_to_model(education)
}

fn convert_active_to_model(
    active_education: education::ActiveModel,
) -> Result<EducationModel, AppError> {
    active_education.try_into_model().map_err(|error| {
        eprintln!(
            "Error converting education's active model to model: {:?}",
            error
        );
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
