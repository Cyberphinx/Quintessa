use crate::utilities::{
    app_error::AppError,
    hash::{hash_password, verify_password},
    jwt::create_token,
    token_wrapper::TokenWrapper,
};
use application::{
    core::mapping_profiles::convert_user_to_response,
    users::{
        create::create_new_user, details::find_by_email, edit::edit_role, save::save_active_user,
        RequestCreateUser, RequestEditRole, ResponseCreateUser, ResponseDataUser, ResponseEditRole,
    },
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use domain::user::{self};
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

use super::handle_result;

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseCreateUser>, AppError> {
    let mut new_user = user::ActiveModel {
        ..Default::default()
    };
    new_user.email = Set(request_user.email.clone());
    new_user.password = Set(hash_password(&request_user.password)?);
    new_user.role = Set("user".to_owned());

    let response = create_new_user(&db, new_user).await;

    handle_result(response).await
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let user = find_by_email(&db, request_user.email).await;
    match user {
        Ok(user) => {
            if !verify_password(&request_user.password, &user.password)? {
                return Err(AppError::new(
                    StatusCode::UNAUTHORIZED,
                    "Incorrect email and/or password",
                ));
            }

            let token = create_token(&token_secret.0, user.id, user.role.clone())?;

            let mut user = user.into_active_model();

            user.token = Set(Some(token));

            let response = save_active_user(&db, user).await;
            match response {
                Ok(user) => {
                    let response = convert_user_to_response(user);
                    Ok(Json(ResponseDataUser { data: response }))
                }
                Err(error) => {
                    eprintln!("{}", error.message);
                    Err(AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        error.message,
                    ))
                }
            }
        }
        Err(error) => {
            eprintln!("{}", error.message);
            Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                error.message,
            ))
        }
    }
}

pub async fn logout(
    Extension(user): Extension<user::Model>,
    State(db): State<DatabaseConnection>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    let result = save_active_user(&db, user).await;
    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(error) => {
            eprintln!("{}", error.message);
            Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong",
            ))
        }
    }
}

pub async fn edit_role_endpoint(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestEditRole>,
) -> Result<Json<ResponseEditRole>, AppError> {
    handle_result(edit_role(db, request_user).await).await
}
