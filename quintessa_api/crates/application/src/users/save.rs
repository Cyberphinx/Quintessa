use crate::core::app_error::AppError;
use axum::http::StatusCode;
use domain::user::{self, Model as UserModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, TryIntoModel};

pub async fn save_active_user(
    db: &DatabaseConnection,
    user: user::ActiveModel,
) -> Result<UserModel, AppError> {
    let user = user.save(db).await.map_err(|error| {
        let error_message = error.to_string();
        if error_message
            .contains(r#"duplicate key value violates unique constraint "user_email_key""#)
        {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "Username already taken, try again with a different user name",
            )
        } else {
            eprintln!("Error saving user: {:?}", error_message);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error saving user: {:?}", error_message),
            )
        }
    })?;

    convert_active_to_model(user)
}

fn convert_active_to_model(active_user: user::ActiveModel) -> Result<UserModel, AppError> {
    active_user.try_into_model().map_err(|error| {
        eprintln!("Error converting user active model to model: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error converting user active model to model: {:?}", error),
        )
    })
}
