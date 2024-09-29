use pwhash::bcrypt;
use sqlx::{Pool, Postgres};
use crate::models::user::{CreateUserRequest, User};
use crate::routes::database_functions;
use crate::{routes::database_functions::users_db::{create_user, get_user_by_username}, routes::database_functions::athletics_db::read_into_db};

pub async fn seed_database(pool : &Pool<Postgres>){
    seed_users(pool).await;
    read_into_db(pool).await;
}

async fn seed_users(pool : &Pool<Postgres>){
    // Check if the user already exists.

    let email = "jksamson@outlook.com";
    let user: Option<User> = get_user_by_username(pool, &email.to_string()).await;

    if user.is_some() {
        println!("User already exists");
        return;
    }

    // Create the new users
    let hash = bcrypt::hash("Jojo the 123!").unwrap();
    let user_dto: CreateUserRequest
        = CreateUserRequest::new("Jordan".to_string(), "Samson".to_string(), "jkdsamson@outlook.com".to_string(), Some("0452339108".to_string()), "".to_string());
    create_user(pool, user_dto, hash).await;
}