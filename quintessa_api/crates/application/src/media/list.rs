use axum::Json;
use itertools::Itertools;

use crate::core::{app_error::AppError, mapping_profiles::convert_media_to_response};
use ::domain::media::Entity as Media;
use sea_orm::{DatabaseConnection, EntityTrait};

use super::ResponseDataMedias;

pub async fn list(db: &DatabaseConnection) -> Result<Json<ResponseDataMedias>, AppError> {
    let all_media = Media::find()
        .all(db)
        .await?
        .into_iter()
        .map(convert_media_to_response)
        .collect_vec();
    let response = ResponseDataMedias { data: all_media };
    Ok(Json(response))
}
