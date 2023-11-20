use axum::Json;
use domain::project;
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel, Set};

use crate::{
    core::{app_error::AppError, mapping_profiles::convert_project_to_response},
    projects::save::save_active_project,
};

use super::{RequestCreateProject, ResponseDataProject};

pub async fn edit(
    project_id: i32,
    db: &DatabaseConnection,
    request_project: RequestCreateProject,
) -> Result<Json<ResponseDataProject>, AppError> {
    let mut project = project::Entity::find_by_id(project_id)
        .one(db)
        .await?
        .ok_or_else(|| {
            eprintln!("Could not find project by id");
            AppError::new(StatusCode::NOT_FOUND, "Not found")
        })?
        .into_active_model();

    project.updated_at = Set(Some(chrono::Utc::now().into()));
    project.name = Set(request_project.name);
    project.category = Set(request_project.category);
    project.sub_category = Set(request_project.sub_category);
    project.description = Set(request_project.description);
    project.tasks = Set(request_project.tasks);
    project.sector = Set(request_project.sector);
    project.location = Set(request_project.location);
    project.address = Set(request_project.address);
    project.client = Set(request_project.client);
    project.start_date = Set(request_project.start_date);
    project.completion_date = Set(request_project.completion_date);
    project.architect = Set(request_project.architect);
    project.main_contractor = Set(request_project.main_contractor);
    project.project_manager = Set(request_project.project_manager);
    project.structural_engineer = Set(request_project.structural_engineer);
    project.services_engineer = Set(request_project.services_engineer);

    let saved_project = save_active_project(db, project).await?;

    let response = convert_project_to_response(saved_project);

    Ok(Json(ResponseDataProject { data: response }))
}
