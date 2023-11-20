use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub role: String,
}

pub fn create_token(secret: &str, user_id: i32, role: String) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let expires_at = now + Duration::days(7);
    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp() as usize,
        exp: expires_at.timestamp() as usize,
        role,
    };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&token_header, &claims, &key).map_err(|error| {
        eprintln!("Error creating token: {:?}", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error, please try again later",
        )
    })
}

pub fn create_refresh_token() -> Result<String, AppError> {
    Ok(Uuid::new_v4().to_string())
}

pub fn validate_token(secret: &str, token: &str) -> Result<String, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    // jwt automatically adds 5 minutes to the end of the expiration time
    // so we reset this clock skew leeway to zero to remove it
    validation.leeway = 0;
    let result = decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
            }
            _ => {
                eprintln!("Error validating token: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
            }
        })?
        .claims;
    Ok(result.sub)
}

pub fn validate_admin_token(secret: &str, token: &str) -> Result<String, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    // jwt automatically adds 5 minutes to the end of the expiration time
    // so we reset this clock skew leeway to zero to remove it
    validation.leeway = 0;
    let result = decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
            }
            _ => {
                eprintln!("Error validating token: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
            }
        })?
        .claims;

    // println!("{}", &result.role);

    if result.role == "admin" {
        Ok(result.sub)
    } else {
        Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not permitted to do this!",
        ))
    }
}
