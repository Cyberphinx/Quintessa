use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, TryIntoModel};

use domain::resume::{self, Model as ResumeModel};

use crate::core::app_error::AppError;

pub async fn save_active_resume(
    db: &DatabaseConnection,
    resume: resume::ActiveModel,
) -> Result<ResumeModel, AppError> {
    let resume = resume.save(db).await.map_err(|error| {
        eprintln!("Error saving resume: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again",
        )
    })?;

    convert_active_to_model(resume)
}

fn convert_active_to_model(active_resume: resume::ActiveModel) -> Result<ResumeModel, AppError> {
    active_resume.try_into_model().map_err(|error| {
        eprintln!(
            "Error converting resume's active model to model: {:?}",
            error
        );
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
