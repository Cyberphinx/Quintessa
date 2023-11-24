use application::{
    educations::{self, RequestCreateEducation, ResponseDataEducation},
    resume::{
        create::create, details::details, RequestCreateResume, ResponseDataResume,
        ResponseDataResumeDto,
    },
    works::{self, RequestCreateWork, ResponseDataWork},
};
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::DatabaseConnection;

use crate::utilities::app_error::AppError;

use super::handle_result;

pub async fn create_resume(
    State(db): State<DatabaseConnection>,
    Json(request_resume): Json<RequestCreateResume>,
) -> Result<Json<ResponseDataResume>, AppError> {
    handle_result(create(&db, request_resume).await).await
}

pub async fn get_resume(
    Path(resume_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataResumeDto>, AppError> {
    handle_result(details(resume_id, &db).await).await
}

pub async fn create_work(
    State(db): State<DatabaseConnection>,
    Json(request_work): Json<RequestCreateWork>,
) -> Result<Json<ResponseDataWork>, AppError> {
    handle_result(works::create::create(&db, request_work).await).await
}

pub async fn create_education(
    State(db): State<DatabaseConnection>,
    Json(request_education): Json<RequestCreateEducation>,
) -> Result<Json<ResponseDataEducation>, AppError> {
    handle_result(educations::create::create(&db, request_education).await).await
}
