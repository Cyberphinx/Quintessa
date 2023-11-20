use crate::utilities::app_error::AppError;
use application::projects::{
    create::create,
    delete::delete_project,
    details::find_project_by_id,
    edit::edit,
    list::{get_all_projects_with_media, get_projects_totals},
    project_params::ProjectParams,
    RequestCreateProject, ResponseDataProject, ResponseDataProjectDto, ResponseDataProjectDtos,
    ResponseTotals,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use sea_orm::DatabaseConnection;

use super::{handle_result, handle_void_result};

pub async fn create_project(
    State(db): State<DatabaseConnection>,
    Json(request_project): Json<RequestCreateProject>,
) -> Result<Json<ResponseDataProject>, AppError> {
    handle_result(create(&db, request_project).await).await
}

pub async fn get_project(
    Path(project_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataProjectDto>, AppError> {
    handle_result(find_project_by_id(project_id, &db).await).await
}

pub async fn get_projects(
    Query(project_params): Query<ProjectParams>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataProjectDtos>, AppError> {
    handle_result(get_all_projects_with_media(project_params, &db).await).await
}

pub async fn get_totals(
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseTotals>, AppError> {
    handle_result(get_projects_totals(&db).await).await
}

pub async fn delete_project_endpoint(
    Path(project_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    handle_void_result(delete_project(project_id, &db).await).await
}

pub async fn edit_project(
    Path(project_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Json(request_company): Json<RequestCreateProject>,
) -> Result<Json<ResponseDataProject>, AppError> {
    handle_result(edit(project_id, &db, request_company).await).await
}
