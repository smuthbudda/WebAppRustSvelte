
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenDetails {
    pub token: Option<String>,
    pub token_uuid: uuid::Uuid,
    pub user_id: i32,
    pub expires_in: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub token_uuid: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}
