use askama::filters::{into_f64, into_isize};
use rand_core::le::read_u64_into;
use sqlx::{query_as, Error, Pool, Postgres};

use crate::models::{iaaf_points::PointsInsert, user::{CreateUserRequest, UpdateUserRequest, User, UserIaafPoints}};



/// Creates a new user in the database.
///
/// # Arguments
/// * `pool` - A reference to the Postgres connection pool.
/// * `dto` - A `CreateUserRequest` struct containing the new user information.
/// * `hash` - A string containing the hashed password for the new user.
///
/// # Returns
/// `true` if the user was successfully created, `false` otherwise.
pub async fn create_user(pool: &Pool<Postgres>, dto: CreateUserRequest, hash: String) -> bool {
    let insert_result = sqlx::query(
        r#"INSERT INTO users ( first_name, last_name, email, phone, active, password)
        VALUES ($1, $2, $3, $4, TRUE, $5);
        "#,
    )
    .bind(dto.first_name)
    .bind(dto.last_name)
    .bind(dto.email)
    .bind(dto.phone)
    .bind(hash)
    .execute(pool)
    .await;

    match insert_result {
        Err(_e) => return false,
        Ok(result) => result.rows_affected() == 1,
    }
}

/// Retrieves a user from the database by their username.
///
/// # Arguments
/// * `pool` - A reference to the Postgres connection pool.
/// * `user_name` - A reference to the username of the user to retrieve.
///
/// # Returns
/// An optional `User` struct representing the retrieved user, or `None` if the user was not found.
pub async fn get_user_by_username(pool: &Pool<Postgres>, email: &String) -> Option<User> {
    let user: Option<User> = sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE email = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .expect("Error loading user.");
    user
}

/// Updates an existing user in the database.
///
/// # Arguments
/// * `pool` - A reference to the Postgres connection pool.
/// * `dto` - A `CreateUserRequest` struct containing the updated user information.
/// * `id` - A reference to the UUID of the user to be updated.
///
/// # Returns
/// An optional `User` struct representing the updated user, or `None` if the update failed.
pub async fn update_user(
    pool: &Pool<Postgres>,
    dto: UpdateUserRequest,
    id: &i32,
) -> Option<User> {
    let user: Option<User> = sqlx::query_as(
        r#"UPDATE users
        SET email = $1, first_name = $2, last_name = $3, phone = $4
        WHERE id = $5
        RETURNING *"#,
    )
    .bind(dto.email)
    .bind(dto.first_name)
    .bind(dto.last_name)
    .bind(dto.phone)
    .bind(id)
    .fetch_optional(pool)
    .await
    .expect("Error updating user.");

    user
}

/// Retrieves a user from the database by their unique identifier (UUID).
///
/// # Arguments
/// * `pool` - A reference to the Postgres connection pool.
/// * `id` - A reference to the UUID of the user to retrieve.
///
/// # Returns
/// An optional `User` struct representing the retrieved user, or `None` if the user was not found.
pub async fn get_user_by_id(pool: &Pool<Postgres>, id: &i32) -> Option<User> {
    let user: Option<User> = sqlx::query_as(
        r#"SELECT * FROM users 
                WHERE id = $1
                FETCH FIRST 1 ROWS ONLY"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .expect("Error loading user.");

    user
}

pub async fn get_user_points(
    pool: &Pool<Postgres>,
    user_id: i32,
) -> Result<Vec<PointsInsert>, Error> {
    let user_points: Vec<PointsInsert> = query_as(
        r#"
        SELECT p.*
        FROM user_points up
        JOIN points p ON p.id = up.point_id
        WHERE up.user_id = $1
        "#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(user_points)
}

pub async fn insert_new_user_points(pool: &Pool<Postgres>, user_id: &i32, point_id: &i32) -> Result<bool, Error> {
    //Check one does not exist
    let user_points: Option<PointsInsert> = query_as(
        r#"
        SELECT *
        FROM user_points 
        WHERE user_id = $1 AND point_id = $2
        "#
    )
    .bind(user_id)
    .bind(point_id)
    .fetch_optional(pool)
    .await?;

    if user_points.is_some() {
        return Err(sqlx::Error::RowNotFound);
    }

    let insert_result = sqlx::query(
        r#"INSERT INTO user_points (user_id, point_id) 
        VALUES ($1, $2);
        "#,
    )
    .bind(user_id)
    .bind(point_id)
    .execute(pool)
    .await?;

    return Ok(insert_result.rows_affected() == 1);
}


pub async fn delete_user_points(pool: &Pool<Postgres>, user_id: &i32, point_id: &i32) -> Result<bool, Error> {
    //Check one does not exist
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM user_points
        WHERE user_id = $1 AND point_id = $2
        "#
    )
    .bind(user_id)
    .bind(point_id)
    .fetch_one(pool)
    .await?;

    if count.0 == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    let insert_result = sqlx::query(
        r#"DELETE FROM user_points WHERE user_id = $1 AND point_id = $2;
        "#,
    )
    .bind(user_id)
    .bind(point_id)
    .execute(pool)
    .await?;

    return Ok(insert_result.rows_affected() == 1);
}
