use application::media::{
    create::create, delete::delete_media, edit::edit, list::list, RequestCreateMedia,
    RequestEditMedia, ResponseDataMedia, ResponseDataMedias,
};
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::DatabaseConnection;

use crate::utilities::app_error::AppError;

use super::{handle_result, handle_void_result};

pub async fn create_media(
    State(db): State<DatabaseConnection>,
    Json(request_media): Json<RequestCreateMedia>,
) -> Result<Json<ResponseDataMedia>, AppError> {
    handle_result(create(&db, request_media).await).await
}

pub async fn edit_media(
    State(db): State<DatabaseConnection>,
    Json(request_media): Json<RequestEditMedia>,
) -> Result<Json<ResponseDataMedia>, AppError> {
    handle_result(edit(&db, request_media).await).await
}

pub async fn list_media(
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataMedias>, AppError> {
    handle_result(list(&db).await).await
}

pub async fn delete_media_endpoint(
    Path(media_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    handle_void_result(delete_media(media_id, &db).await).await
}
