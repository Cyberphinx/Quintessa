use domain::media::{self, Model as MediaModel};
use reqwest::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, TryIntoModel};

use crate::core::app_error::AppError;

pub async fn save_active_media(
    db: &DatabaseConnection,
    media: media::ActiveModel,
) -> Result<MediaModel, AppError> {
    let media = media.save(db).await.map_err(|error| {
        eprintln!("Error creating media: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong, please try again",
        )
    })?;

    convert_active_to_model(media)
}

fn convert_active_to_model(active_media: media::ActiveModel) -> Result<MediaModel, AppError> {
    active_media.try_into_model().map_err(|error| {
        eprintln!("Error converting media active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
