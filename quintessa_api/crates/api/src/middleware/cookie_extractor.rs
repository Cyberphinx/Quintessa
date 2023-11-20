// use axum::{
//     http::{HeaderMap, Request},
//     middleware::Next,
//     response::Response,
//     TypedHeader,
// };
// use reqwest::{header::COOKIE, StatusCode};
//
// use crate::utilities::app_error::AppError;
//
// pub async fn cookie_extractor<T>(
//     TypedHeader(headers): TypedHeader<HeaderMap>,
//     mut request: Request<T>,
//     next: Next<T>,
// ) -> Result<Response, AppError> {
//     if let Some(cookie) = &headers.get(COOKIE) {
//         request.extensions_mut().insert(cookie.to_owned());
//     } else {
//         // if user is not logged in, return early with unauthorized
//         return Err(AppError::new(
//             StatusCode::UNAUTHORIZED,
//             "You are not authorized for this",
//         ));
//     }
//
//     Ok(next.run(request).await)
// }
