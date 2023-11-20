use axum::Json;
use domain::user;
use sea_orm::DatabaseConnection;

use crate::core::app_error::AppError;

use super::{save::save_active_user, ResponseCreateUser};

pub async fn create_new_user(
    db: &DatabaseConnection,
    new_user: user::ActiveModel,
) -> Result<Json<ResponseCreateUser>, AppError> {
    let user = save_active_user(db, new_user).await?;

    let response_user = ResponseCreateUser {
        id: user.id,
        email: user.email,
        role: user.role,
    };

    Ok(Json(response_user))
}
