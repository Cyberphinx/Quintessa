use axum::http::StatusCode;
use domain::user::{self, Entity as Users, Model as UserModel};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::core::app_error::AppError;

pub async fn find_by_email(db: &DatabaseConnection, email: String) -> Result<UserModel, AppError> {
    Users::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by email: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error logging in, please try again later",
            )
        })?
        .ok_or_else(|| AppError::new(StatusCode::BAD_REQUEST, "incorrect email and/or password"))
}
