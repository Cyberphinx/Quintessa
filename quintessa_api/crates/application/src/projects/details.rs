use axum::Json;
use domain::{
    media::{Entity as Medias, Model as MediaModel},
    project::{Entity as Projects, Model as ProjectModel},
};
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};

use crate::core::{app_error::AppError, mapping_profiles::convert_project_to_dto};

use super::ResponseDataProjectDto;

pub async fn details(
    project_id: i32,
    db: &DatabaseConnection,
) -> Result<(ProjectModel, Vec<MediaModel>), AppError> {
    let project = Projects::find_by_id(project_id)
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting project by id: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting project, please try again later",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::NOT_FOUND, "The project is not found"))?;

    let medias = project
        .find_related(Medias)
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("Problem finding related media: {}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Problem finding related media",
            )
        })?;

    Ok((project, medias))
}

pub async fn find_project_by_id(
    project_id: i32,
    db: &DatabaseConnection,
) -> Result<Json<ResponseDataProjectDto>, AppError> {
    let project_tuple = details(project_id, db).await?;
    Ok(Json(ResponseDataProjectDto {
        data: convert_project_to_dto(project_tuple),
    }))
}
