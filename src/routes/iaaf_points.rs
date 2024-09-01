use crate::models::iaaf_points::{Category, Gender, PointsInsert, PointsSearchQueryParams};
use axum::{
    extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Extension, Json
};
use axum_login::AuthUser;
use serde_json;
use std::{error::Error, sync::Arc, time::SystemTime};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

use super::{database_functions::users_db::{delete_user_points, get_user_points, insert_new_user_points}, jwt_auth::JWTAuthMiddleware, routes::AppState};

static FILE_LOCATION: &str = "data/WorldAthletics.json";

pub async fn read_iaaf_json(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(id) FROM points"#)
        .fetch_one(&data.db)
        .await
        .unwrap();

    let mut json_response = serde_json::json!({
        "Message": "Points Already Exists"
    });

    if count > 200000 {
        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }

    let models: Result<Vec<PointsInsert>, Box<dyn Error>> = read_file_async().await;

    match models {
        Err(e) => {
            let error = "Error: ".to_string() + &e.to_string();
            json_response = serde_json::json!({
                "Message": error
            });
            return Ok(Json(json_response));
        }
        Ok(_) => {
            print!("inserting into database");
            //There has to be a better way to insert 200000 records than individually. It takes so long.
            let models: Vec<PointsInsert> = models.unwrap_or_default();

            let points: Vec<i32> = models.iter().map(|p| p.points).collect();
            let genders: Vec<String> = models.iter().map(|p| p.gender.clone()).collect();
            let categories: Vec<String> = models.iter().map(|p| p.category.clone()).collect();
            let events: Vec<String> = models.iter().map(|p| p.event.clone()).collect();
            let marks: Vec<f64> = models.iter().map(|p| p.mark).collect();
            let start = SystemTime::now();
            let query_result = sqlx::query(
            r#"INSERT INTO points (points, gender, category, event, mark)
                    SELECT * FROM UNNEST($1::INTEGER[], $2::VARCHAR(10)[], $3::VARCHAR(20)[], $4::VARCHAR(10)[], $5::Float[])"#,
            )
            .bind(points)
            .bind(genders)
            .bind(categories)
            .bind(events)
            .bind(marks)
            .execute(&data.db)
            .await;

            match query_result {
                Ok(_) => {}
                Err(err) => {
                    println!("Error: {}", err);
                }
            }

            json_response = serde_json::json!({
                "Status" : "Values Added!",
                "Time": start.elapsed().unwrap().as_secs_f32()
            });

            return Ok(Json(json_response));
        }
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

async fn read_file_async() -> Result<Vec<PointsInsert>, Box<dyn Error>> {
    let file = File::open(FILE_LOCATION).await?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).await?;

    let points: Vec<PointsInsert> = serde_json::from_slice(&buffer)?;

    Ok(points)
}
