use axum::Json;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use crate::core::app_error::AppError;

use super::{details::find_by_email, save::save_active_user, RequestEditRole, ResponseEditRole};

pub async fn edit_role(
    db: DatabaseConnection,
    request: RequestEditRole,
) -> Result<Json<ResponseEditRole>, AppError> {
    let mut user = find_by_email(&db, request.email).await?.into_active_model();

    user.role = Set(request.role);

    let saved_user = save_active_user(&db, user).await?;

    let response = ResponseEditRole {
        email: saved_user.email,
        role: saved_user.role,
    };

    Ok(Json(response))
}
