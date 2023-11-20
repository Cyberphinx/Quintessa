use axum::http::StatusCode;
use domain::project::Entity as Projects;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};

use crate::core::app_error::AppError;

pub async fn delete_project(project_id: i32, db: &DatabaseConnection) -> Result<(), AppError> {
    let project = if let Some(project) =
        Projects::find_by_id(project_id)
            .one(db)
            .await
            .map_err(|error| {
                eprintln!("Error finding project by id: {}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
            })? {
        project.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Link not found"));
    };

    let _delete_result = Projects::delete(project.to_owned().into_active_model())
        .exec(db)
        .await
        .map_err(|error| {
            eprintln!("Error deleting project (id: {:?}): {}", &project.id, error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?;

    Ok(())
}
