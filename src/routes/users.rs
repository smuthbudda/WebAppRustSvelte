use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    models::user::{CreateUserRequest, UpdateUserRequest},
    routes::utils::constants::*,
};

use super::{
    database_functions::users_db::{create_user, get_user_by_username, update_user},
    jwt_auth::JWTAuthMiddleware,
    routes::AppState,
};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct LoginRequest {
    user_name: String,
    password: String,
}

pub async fn get_users_handler(
    State(_data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status": "ok",
    });

    Ok(Json(json_response))
}

pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match get_user_by_username(&data.db, &req.user_name).await {
        None => println!("user does not exist"),
        Some(_user) => return Ok(Json(serde_json::json!({"status":"User Already Exists"}))),
    }

    let hash = hash_password(&req.password);

    match create_user(&data.db, req, hash).await {
        true => Ok(Json(
            serde_json::json!({"status":RESPONSE_STATUS_SUCCESS, "Message":"User Created"}),
        )),
        false => Ok(Json(serde_json::json!({"status":"Error inserting"}))),
    }
}

pub async fn get_user_details_handler(
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  RESPONSE_STATUS_SUCCESS,
        "data": serde_json::json!({
            "user": &jwtauth.user.to_dto()
        })
    });

    Ok(Json(json_response))
}
///
/// Update user details
///     - Only the user can update their own details
pub async fn update_user_handler(
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
    Path(user_id): Path<i32>,
    State(data): State<Arc<AppState>>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Check if the user ID in the JWT matches the user ID in the request
    if jwtauth.user.id != user_id {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "status": "User ID does not match." })),
        ));
    }
    // Check if the user exists
    match get_user_by_username(&data.db, &req.user_name).await {
        None => println!("user does not exist"),
        Some(_user) => return Ok(Json(serde_json::json!({"status":"User Already Exists"}))),
    }

    // Update the user
    match update_user(&data.db, req, &user_id).await {
        None => {
            Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "status": "Error updating user." })),
            ))
        }
        Some(user) => {
            let json_response = serde_json::json!({
                "status":  RESPONSE_STATUS_SUCCESS,
                "data": serde_json::json!({
                    "user": user
                })
            });
            Ok(Json(json_response))
        }
    }
}

fn hash_password(password: &str) -> String {
    bcrypt::hash(password).unwrap()
}
