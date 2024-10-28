use super::{jwt_auth::JWTAuthMiddleware, routes::AppState, utils::token};
use crate::models::{token::TokenDetails, user::User};
use axum::{
    extract::State,
    http::{header, HeaderMap, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LoginRequest {
    email: String,
    password: String,
}

/// Handles the login request by verifying the user's credentials and generating access and refresh tokens.
///
/// This function takes a `LoginRequest` object containing the user's username and password, and returns a successful response with access and refresh tokens if the credentials are valid, or an error response if the credentials are invalid or there is a database error.
///
/// The function performs the following steps:
/// 1. Fetches the user from the database based on the provided username.
/// 2. Verifies the user's password using the bcrypt library.
/// 3. Generates access and refresh tokens using the `generate_token` function.
/// 4. Caches the generated tokens using the `cache_jwt_token` function.
/// 5. Sets the access and refresh tokens as cookies in the response.
/// 6. Returns the successful response with the access token.
pub async fn login_handler(
    State(data): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user: Option<crate::models::user::User> = sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE email = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(&req.email)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if user.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"status":"Error user not found."})),
        ));
    }

    let user = user.unwrap();

    let is_match = bcrypt::verify(&req.password, &user.get_hash());

    if !is_match {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid email or password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }
    let access_token_details = generate_token(
        user.id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    )?;
    let refresh_token_details = generate_token(
        user.id,
        data.env.refresh_token_max_age,
        data.env.refresh_token_private_key.to_owned(),
    )?;

    cache_jwt_token(&data, &access_token_details).await;
    cache_jwt_token(&data, &refresh_token_details).await;

    let access_cookie = Cookie::build((
        "access_token",
        access_token_details.token.clone().unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let refresh_cookie = Cookie::build((
        "refresh_token",
        refresh_token_details.token.unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(data.env.refresh_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .http_only(false);

    let mut response = Response::new(
        json!({"status": "success", "access_token": access_token_details.token.unwrap()})
            .to_string(),
    );
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        refresh_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    response.headers_mut().extend(headers);

    Ok(response)
}

pub async fn logout_handler(
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Remove the token from the cache
    remove_jwt_token(&data, &jwtauth.access_token_uuid).await;
    let response = Response::new(json!({"status": "success", "":""}).to_string());
    Ok(response)
}
/// Handles the refresh of an access token by verifying the provided refresh token and generating a new access token.
///
/// This handler is responsible for the following steps:
/// 1. Retrieve the refresh token from the cookie jar.
/// 2. Verify the refresh token using the configured refresh token public key.
/// 3. Fetch the user associated with the verified refresh token from the database.
/// 4. Generate a new access token for the user.
/// 5. Cache the new access token in the application state.
/// 6. Set the new access token and a "logged_in" cookie in the response.
///
/// If any of the steps fail, the handler will return an appropriate error response.
pub async fn refresh_access_token_handler(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "could not refresh access token"
            });
            (StatusCode::FORBIDDEN, Json(error_response))
        })?;

    let refresh_token_details =
        match token::verify_jwt_token(data.env.refresh_token_public_key.to_owned(), &refresh_token)
        {
            Ok(token_details) => token_details,
            Err(e) => {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format_args!("{:?}", e)
                });
                return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
            }
        };

    let cache = &data.cache;
    let cache_token = cache.get(&refresh_token_details.token_uuid).await.unwrap();

    let user: Option<User> = sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE id = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(cache_token.user_id)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if user.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"status":"Error user not found."})),
        ));
    }

    let access_token_details: TokenDetails = generate_token(
        user.unwrap().id,
        data.env.access_token_max_age,
        data.env.access_token_private_key.to_owned(),
    )?;

    let access_cookie = Cookie::build((
        "access_token",
        access_token_details.token.clone().unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(data.env.access_token_max_age * 60))
        .same_site(SameSite::Lax)
        .same_site(SameSite::Lax)
        .http_only(false);

    cache_jwt_token(&data, &access_token_details).await;

    let mut response = Response::new(
        json!({"status": "success", "access_token": access_token_details.token.unwrap()})
            .to_string(),
    );
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );
    response.headers_mut().extend(headers);
    Ok(response)
}

/// Generates a JWT token for the given user ID with the specified maximum age and private key.
///
/// # Arguments
/// * `user_id` - The ID of the user to generate the token for.
/// * `max_age` - The maximum age of the token in minutes.
/// * `private_key` - The private key to use for signing the token.
///
/// # Returns
/// A `TokenDetails` struct containing the generated token, or an error if the token could not be generated.
fn generate_token(
    user_id: i32,
    max_age: i64,
    private_key: String,
) -> Result<TokenDetails, (StatusCode, Json<serde_json::Value>)> {
    token::generate_jwt_token(user_id, max_age, private_key).map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("error generating token: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })
}

fn get_token_details(
    token : String,
    private_key: String
) -> Result<TokenDetails, (StatusCode, Json<serde_json::Value>)> {
    token::verify_jwt_token(private_key, &token).map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("error generating token: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })
}

/// Caches a JWT token in the application state.
///
/// This function takes the application state and a `TokenDetails` struct, and inserts the token into the cache using the token's UUID as the key.
///
/// # Arguments
/// * `data` - A reference to the `AppState` struct containing the application state.
/// * `token` - A reference to the `TokenDetails` struct containing the token to be cached.
async fn cache_jwt_token(data: &Arc<AppState>, token: &TokenDetails) {
    data.cache.insert(token.token_uuid, token.clone()).await;
}

/// Removes a JWT token from the application state cache.
///
/// This function takes the application state and a `TokenDetails` struct, and removes the token from the cache using the token's UUID as the key.
///
/// # Arguments
/// * `data` - A reference to the `AppState` struct containing the application state.
/// * `token` - A reference to the `TokenDetails` struct containing the token to be removed.
async fn remove_jwt_token(data: &Arc<AppState>, token: &uuid::Uuid) {
    data.cache.remove(&token).await;
}
