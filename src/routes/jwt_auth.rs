use super::{routes::AppState, utils::{constants::*, token}};
use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use std::sync::Arc;
use crate::models::user::User;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddleware {
    pub user: User,
    pub access_token_uuid: uuid::Uuid,
}


/// Handles JWT-based authentication for the application.
///
/// This function extracts the access token from the request, verifies it, and retrieves the associated user information.
/// If the token is valid and the user exists, the function attaches the user and access token details to the request extensions,
/// allowing them to be accessed by subsequent middleware or route handlers.
/// If the token is invalid or the user is not found, the function returns an error response with the appropriate status code and error message.
/// Handles JWT-based authentication for the application.
pub async fn auth(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let access_token = cookie_jar
        .get(ACCESS_TOKEN)
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let access_token = access_token.ok_or_else(|| {
        let error_response = ErrorResponse {
            status: RESPONSE_STATUS_FAIL,
            message: NOT_LOGGED_IN.to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let access_token_details =
        match token::verify_jwt_token(data.env.access_token_public_key.to_owned(), &access_token) {
            Ok(token_details) => token_details,
            Err(e) => {
                let error_response = ErrorResponse {
                    status: RESPONSE_STATUS_FAIL,
                    message: format!("{:?}", e),
                };
                return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
            }
        };

    let access_token_uuid = uuid::Uuid::parse_str(&access_token_details.token_uuid.to_string())
        .map_err(|_| {
            let error_response = ErrorResponse {
                status: RESPONSE_STATUS_FAIL,
                message: INVALID_TOKEN.to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?;

    // Get the token from the cache
    let cache = &data.cache;
    let cache_token: Option<crate::models::token::TokenDetails> = cache.get(&access_token_uuid).await;

    if cache_token.is_none(){
        let error_response = ErrorResponse {
            status: RESPONSE_STATUS_FAIL,
            message: TOKEN_NOT_FOUND.to_string(),
        };
        return Err((StatusCode::UNAUTHORIZED, Json(error_response)))
    }


    // if  cache_token.as_ref().unwrap().expires_in.is_some(){
    //     match  cache_token.as_ref().unwrap().expires_in.unwrap() >= chrono::Utc::now().timestamp(){
    //         true => {

    //         },
    //         false => {

    //         }
    //     }
    // }

    // Get the user from the database
    let user: Option<User> = query_as(
        r#"SELECT * FROM users 
                WHERE id = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(cache_token.unwrap().user_id)
    .fetch_optional(&data.db)
    .await
    .map_err(|_| {
        let error_response = ErrorResponse {
            status: RESPONSE_STATUS_FAIL,
            message: USER_NOT_FOUND.to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    let user = user.ok_or_else(|| {
        let error_response: ErrorResponse = ErrorResponse {
            status: RESPONSE_STATUS_FAIL,
            message: USER_NOT_FOUND.to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    req.extensions_mut().insert(JWTAuthMiddleware {
        user,
        access_token_uuid,
    });
    Ok(next.run(req).await)
}
