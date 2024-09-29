use serde::{Deserialize, Serialize};
use super::iaaf_points::PointsInsert;

#[derive(Default, Debug, Clone, PartialEq,  Deserialize, sqlx::FromRow, Serialize, Eq)]
pub struct User {
    pub id: i32,
    pub active: bool,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    password: String
}

#[derive(Default, Debug, Clone, PartialEq,  Deserialize, Serialize, Eq)]
pub struct UserDto {
    pub id: i32,
    pub active: bool,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
}

impl User {
    pub fn get_hash(&self) -> &String {
        &self.password
    }

    pub fn to_dto(&self) -> UserDto {
        UserDto {
            id: self.id,
            active: self.active,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
        }
    }
}
impl axum_login::AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct UserUpdateQueryParams {
    pub id: i32,
}

#[derive(Default, Debug, Clone, PartialEq,  Deserialize, Serialize)]
pub struct UserIaafPoints{
    pub user_id: i32,
    pub point: PointsInsert,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UpdateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserResponse{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}