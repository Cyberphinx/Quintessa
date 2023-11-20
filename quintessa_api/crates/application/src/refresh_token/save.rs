use crate::core::app_error::AppError;
use axum::http::StatusCode;
use domain::refresh_token::{self, Model as RefreshTokenModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, TryIntoModel};

pub async fn save_active_refresh_token(
    db: &DatabaseConnection,
    refresh_token: refresh_token::ActiveModel,
) -> Result<RefreshTokenModel, AppError> {
    let refresh_token = refresh_token.save(db).await.map_err(|error| {
        eprintln!("Error saving refresh token: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error saving refresh token: {:?}", error),
        )
    })?;

    convert_active_to_model(refresh_token)
}

fn convert_active_to_model(
    active_user: refresh_token::ActiveModel,
) -> Result<RefreshTokenModel, AppError> {
    active_user.try_into_model().map_err(|error| {
        eprintln!(
            "Error converting refresh token active model to model: {:?}",
            error
        );
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "Error converting refresh token active model to model: {:?}",
                error
            ),
        )
    })
}
