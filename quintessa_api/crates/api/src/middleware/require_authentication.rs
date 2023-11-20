use crate::utilities::{
    app_error::AppError,
    jwt::{validate_admin_token, validate_token},
    token_wrapper::TokenWrapper,
};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::Request,
    middleware::Next,
    response::Response,
    TypedHeader,
};
use domain::user::Entity as Users;
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn require_authentication<T>(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let bearer_token = token.token();

    // return early if token is not valid
    let user_id_string = validate_token(&token_secret.0, bearer_token)?;
    let user_id = user_id_string.parse::<i32>().map_err(|error| {
        eprintln!("Problem parsing token claims sub to i32: {}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was a problem getting your account id",
        )
    })?;

    // get the user as a model
    let user = Users::find_by_id(user_id).one(&db).await.map_err(|error| {
        eprintln!("Error getting user by token: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was a problem getting your account",
        )
    })?;

    // let user = Users::find()
    //     .filter(user::Column::Token.eq(Some(bearer_token.to_owned())))
    //     .one(&db)
    //     .await
    //     .map_err(|error| {
    //         eprintln!("Error getting user by token: {:?}", error);
    //         AppError::new(
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "There was a problem getting your account",
    //         )
    //     })?;

    // if token doesn't exist, you are logged out
    if let Some(user) = user {
        // if user is logged in, add the user to the request in an extension
        request.extensions_mut().insert(user);
    } else {
        // if user is not logged in, return early with unauthorized
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized for this",
        ));
    }

    Ok(next.run(request).await)
}

pub async fn require_admin_authentication<T>(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let bearer_token = token.token();

    // return early if token is not valid
    let user_id_string = validate_admin_token(&token_secret.0, bearer_token)?;
    let user_id = user_id_string.parse::<i32>().map_err(|error| {
        eprintln!("Problem parsing token claims sub to i32: {}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was a problem getting your account id",
        )
    })?;

    // get the user as a model
    let user = Users::find_by_id(user_id).one(&db).await.map_err(|error| {
        eprintln!("Error getting user by token: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was a problem getting your account",
        )
    })?;

    // let user = Users::find()
    //     .filter(user::Column::Token.eq(Some(bearer_token.to_owned())))
    //     .one(&db)
    //     .await
    //     .map_err(|error| {
    //         eprintln!("Error getting user by token: {:?}", error);
    //         AppError::new(
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "There was a problem getting your account",
    //         )
    //     })?;

    // if token doesn't exist, you are logged out
    if let Some(user) = user {
        // if user is logged in, add the user to the request in an extension
        request.extensions_mut().insert(user);
    } else {
        // if user is not logged in, return early with unauthorized
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized for this",
        ));
    }

    Ok(next.run(request).await)
}
