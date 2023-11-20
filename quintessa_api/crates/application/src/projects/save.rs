use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, TryIntoModel};

use domain::project::{self, Model as ProjectModel};

use crate::core::app_error::AppError;

pub async fn save_active_project(
    db: &DatabaseConnection,
    project: project::ActiveModel,
) -> Result<ProjectModel, AppError> {
    let project = project.save(db).await.map_err(|error| {
        eprintln!("Error saving project: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again",
        )
    })?;

    convert_active_to_model(project)
}

fn convert_active_to_model(active_project: project::ActiveModel) -> Result<ProjectModel, AppError> {
    active_project.try_into_model().map_err(|error| {
        eprintln!(
            "Error converting project active model to model: {:?}",
            error
        );
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
