use axum::{extract::State, Json};
use axum::http::StatusCode;
use sqlx::PgPool;
use crate::models::user::RegisterUser;
use crate::models::user::LoginUser;
use crate::models::user::User;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use axum::debug_handler; 

#[debug_handler]
pub async fn reg(
    State(pool):State<PgPool>,
    Json(payload): Json<RegisterUser>,
) -> Result<Json<User>, StatusCode> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();
    
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email, username, password_hash, created_at,
              is_lock, is_vip, is_kyc_pass, updated_at, is_partner, city
        "#,
        payload.email,
        payload.username,
        password_hash,
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(user))
}

pub async fn login(
    State(pool):State<PgPool>,
    Json(payload): Json<LoginUser>
) -> Result<Json<String>, StatusCode> {
    let record = sqlx::query!(
        "SELECT password_hash FROM users WHERE email = $1",
        payload.email
    )
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let Some(row) = record else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    
    let parsed_hash = PasswordHash::new(&row.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    Ok(Json("Login success".to_string()))
}