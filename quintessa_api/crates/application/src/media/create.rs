use axum::Json;
use domain::media;
use sea_orm::{DatabaseConnection, Set};

use crate::core::{app_error::AppError, mapping_profiles::convert_media_to_response};

use super::{save::save_active_media, RequestCreateMedia, ResponseDataMedia};

pub async fn create(
    db: &DatabaseConnection,
    request_media: RequestCreateMedia,
) -> Result<Json<ResponseDataMedia>, AppError> {
    let new_media = media::ActiveModel {
        project_id: Set(request_media.project_id),
        created_at: Set(Some(chrono::Utc::now().into())),
        media_type: Set(request_media.media_type),
        url: Set(request_media.url),
        caption: Set(request_media.caption),
        description: Set(request_media.description),
        ..Default::default()
    };

    let media = save_active_media(db, new_media).await?;

    let response = convert_media_to_response(media);
    Ok(Json(ResponseDataMedia { data: response }))
}
