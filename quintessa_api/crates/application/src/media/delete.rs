use axum::http::StatusCode;
use domain::media::Entity as Media;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};

use crate::core::app_error::AppError;

pub async fn delete_media(media_id: i32, db: &DatabaseConnection) -> Result<(), AppError> {
    let media = if let Some(media) = Media::find_by_id(media_id).one(db).await.map_err(|error| {
        eprintln!("Error finding media by id: {}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
    })? {
        media.into_active_model()
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "Media not found"));
    };

    let _delete_result = Media::delete(media.to_owned().into_active_model())
        .exec(db)
        .await
        .map_err(|error| {
            eprintln!("Error deleting media (id: {:?}): {}", &media.id, error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
        })?;

    Ok(())
}
