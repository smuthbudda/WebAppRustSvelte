#![allow(dead_code)]
use std::sync::Arc;

use crate::models::token::TokenDetails;

use super::{
    auth::{login_handler, logout_handler, refresh_access_token_handler},
    files::upload_file,
    iaaf_points::{add_user_points_handler, delete_user_points_handler, get_user_points_handler, get_value, read_iaaf_json},
    jwt_auth::auth,
    system_info::{get_system_details_handler, realtime_cpu_handler},
    users::{
        create_user_handler, get_user_details_handler, get_users_handler, update_user_handler,
    },
};

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use moka::future::Cache;
use sqlx::{Pool, Postgres};
use tokio::sync::broadcast;
use uuid::Uuid;

pub type Snapshot = Vec<f32>;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: crate::config::Config,
    pub tx: broadcast::Sender<Snapshot>,
    pub cache: Cache<Uuid, TokenDetails>,
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let user_routes: Router<Arc<AppState>> = Router::new()
        .route("/", get(get_users_handler))
        .route("/user_points/:user_id/:points_id", post(add_user_points_handler))
        .route("/user_points/:user_id/:points_id", delete(delete_user_points_handler), )
        .route("/user_points/:user_id", get(get_user_points_handler))
        .route("/me", get(get_user_details_handler))
        .route("/:id", put(update_user_handler))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth));

    let health_check_routes: Router<Arc<AppState>> =
        Router::new().route("/check", get(super::health_check::health_check));

    let points_routes: Router<Arc<AppState>> = Router::new()
        .route("/read", get(read_iaaf_json))
        .route("/points/:category/:gender/:event", get(get_value));

    let system_routes: Router<Arc<AppState>> = Router::new()
        .route("/cpu", get(realtime_cpu_handler)) //web socket
        .route("/details", get(get_system_details_handler));

    let auth_routes: Router<Arc<AppState>> = Router::new()
        .route("/refresh_token", get(refresh_access_token_handler))
        .route("/user", post(create_user_handler))
        .route("/logout", get(logout_handler))
        .route("/login", post(login_handler));

    let file_routes: Router<Arc<AppState>> = Router::new().route("/upload", post(upload_file));

    let router: Router = Router::new()
        .nest("/user", user_routes)
        .nest("/health_check", health_check_routes)
        .nest("/world_aths", points_routes)
        .nest("/system", system_routes)
        .nest("/auth", auth_routes)
        .nest("/files", file_routes)
        .with_state(app_state);

    Router::new().nest("/api", router)
}
