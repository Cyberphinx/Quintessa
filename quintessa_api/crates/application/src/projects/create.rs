use axum::Json;
use domain::project;
use sea_orm::{DatabaseConnection, Set};

use crate::{
    core::{app_error::AppError, mapping_profiles::convert_project_to_response},
    projects::save::save_active_project,
};

use super::{RequestCreateProject, ResponseDataProject};

pub async fn create(
    db: &DatabaseConnection,
    request_project: RequestCreateProject,
) -> Result<Json<ResponseDataProject>, AppError> {
    let new_project = project::ActiveModel {
        created_at: Set(Some(chrono::Utc::now().into())),
        name: Set(request_project.name),
        category: Set(request_project.category),
        sub_category: Set(request_project.sub_category),
        description: Set(request_project.description),
        tasks: Set(request_project.tasks),
        sector: Set(request_project.sector),
        location: Set(request_project.location),
        address: Set(request_project.address),
        client: Set(request_project.client),
        start_date: Set(request_project.start_date),
        completion_date: Set(request_project.completion_date),
        architect: Set(request_project.architect),
        main_contractor: Set(request_project.main_contractor),
        project_manager: Set(request_project.project_manager),
        structural_engineer: Set(request_project.structural_engineer),
        services_engineer: Set(request_project.services_engineer),
        ..Default::default()
    };

    let project = save_active_project(db, new_project).await?;

    let response = convert_project_to_response(project);

    Ok(Json(ResponseDataProject { data: response }))
}
