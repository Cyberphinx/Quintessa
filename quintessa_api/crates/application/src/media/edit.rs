use axum::Json;
use domain::media;
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel, Set};

use crate::core::{app_error::AppError, mapping_profiles::convert_media_to_response};

use super::{save::save_active_media, RequestEditMedia, ResponseDataMedia};

pub async fn edit(
    db: &DatabaseConnection,
    request_media: RequestEditMedia,
) -> Result<Json<ResponseDataMedia>, AppError> {
    let mut media = media::Entity::find_by_id(request_media.id)
        .one(db)
        .await?
        .ok_or_else(|| {
            eprintln!("Could not find company by id");
            AppError::new(StatusCode::NOT_FOUND, "Not found")
        })?
        .into_active_model();

    media.project_id = Set(request_media.project_id);
    media.updated_at = Set(Some(chrono::Utc::now().into()));
    media.media_type = Set(request_media.media_type);
    media.url = Set(request_media.url);
    media.caption = Set(request_media.caption);
    media.description = Set(request_media.description);

    media.updated_at = Set(Some(chrono::Utc::now().into()));

    let saved_media = save_active_media(db, media).await?;

    let response = convert_media_to_response(saved_media);

    Ok(Json(ResponseDataMedia { data: response }))
}
