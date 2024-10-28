use crate::models::iaaf_points::{Category, Gender, PointsInsert, PointsSearchQueryParams};
use axum::{
    extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Extension, Json
};
use serde_json;
use std::sync::Arc;

use super::{database_functions::users_db::{delete_user_points, get_user_points, insert_new_user_points},
            jwt_auth::JWTAuthMiddleware, routes::AppState, database_functions::athletics_db::{read_into_db}};


pub async fn read_iaaf_json(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, Json<serde_json::Value>)> {
        let json_response = serde_json::json!({
            "Status" : "Values Added!",
        });

        match read_into_db(&data.db).await{
            true => Ok(Json(json_response)),
            false => Err((StatusCode::BAD_REQUEST, Json(json_response)))
        }
}



pub async fn get_value(
    Path((category, gender, event)): Path<(Category, Gender, String)>,
    Query(params): Query<PointsSearchQueryParams>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if (params.mark.is_some() && params.points.is_some())
        || (params.mark.is_none() && params.points.is_none())
    {
        let bad_json = serde_json::json!({
            "status": "Bad Request"
        });
        return Err((StatusCode::NOT_FOUND, Json(bad_json)));
    }

    let query_result = sqlx::query_as::<_, PointsInsert>(
        r#"
    SELECT * FROM points 
    WHERE 
        LOWER(category) = LOWER($1) AND 
        LOWER(gender) = LOWER($2) AND 
        LOWER(event) = LOWER($3) AND
        (ROUND(mark::numeric, 2) = $4 OR points = $5)
    ORDER BY CASE 
        WHEN mark IS NULL THEN 1 
        ELSE ABS($4 - 1400) 
    END
    FETCH FIRST 1 ROWS ONLY;"#,
    )
    .bind(category.to_string())
    .bind(gender.to_string())
    .bind(event)
    .bind(params.mark)
    .bind(params.points)
    .fetch_optional(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let json_response = serde_json::json!({
        "points": query_result
    });

    Ok(Json(json_response))
}

pub async fn get_user_points_handler(
    Path(user_id): Path<i32>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    if  user_id == 0 {
        let bad_json = serde_json::json!({
            "status": "Bad Request"
        });
        return Err((StatusCode::NOT_FOUND, Json(bad_json)));
    }

    match get_user_points(&data.db, user_id).await{
        Ok(points) => {
            let json_response = serde_json::json!({
                "user_points": points
            });
            Ok(Json(json_response))
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

pub async fn add_user_points_handler(
    Path((user_id, point_id)): Path<(i32, i32)>,
    State(data): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    if  jwtauth.user.id != user_id {
        let bad_json = serde_json::json!({
            "status": "Bad Request"
        });
        return Err((StatusCode::NOT_FOUND, Json(bad_json)));
    }

    match insert_new_user_points(&data.db,&user_id, &point_id).await{
        Ok(_) => {
            let json_response = serde_json::json!({
                "user_points": "success"
            });
            Ok(Json(json_response))
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

pub async fn delete_user_points_handler(
    Path((user_id, point_id)): Path<(i32, i32)>,
    State(data): State<Arc<AppState>>,
    Extension(jwtauth): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    if  jwtauth.user.id != user_id {
        let bad_json = serde_json::json!({
            "status": "Bad Request"
        });
        return Err((StatusCode::NOT_FOUND, Json(bad_json)));
    }

    match delete_user_points(&data.db,&user_id, &point_id).await{
        Ok(_) => {
            let json_response = serde_json::json!({
                "user_points": "success"
            });
            return Ok(Json(json_response));
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Database error: { }", e),
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    }
}

