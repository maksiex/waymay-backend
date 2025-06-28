use serde::{Deserialize, Serialize};
use sqlx::types::time::PrimitiveDateTime;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: Option<PrimitiveDateTime>,
    pub updated_at: Option<PrimitiveDateTime>,
    pub is_lock: Option<bool>,
    pub is_vip: Option<bool>,
    pub is_kyc_pass: Option<bool>,
    pub is_partner: Option<bool>,
    pub city: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub username: String,
    pub password: String,
}
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

