use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use axum::debug_handler;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use sqlx::Row;

use argon2::{
    Argon2,
    PasswordHasher,
    PasswordVerifier,
    password_hash::{SaltString, PasswordHash, rand_core::OsRng},
};

use crate::{
    state::AppState,
    models::jwt::generate_token,
};

#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[debug_handler]
pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<StatusCode, StatusCode> {

    let salt = SaltString::generate(&mut OsRng);

    let hash = match Argon2::default().hash_password(payload.password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let result = sqlx::query(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)"
    )
    .bind(payload.username)
    .bind(hash)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[debug_handler]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {

    let row = match sqlx::query(
        "SELECT id, password_hash FROM users WHERE username = $1"
    )
    .bind(&payload.username)
    .fetch_one(&state.db)
    .await
    {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let user_id: i32 = match row.try_get("id") {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let password_hash: String = match row.try_get("password_hash") {
        Ok(p) => p,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let parsed_hash = match PasswordHash::new(&password_hash) {
        Ok(ph) => ph,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = match generate_token(&user_id.to_string()) {
        Ok(t) => t,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(AuthResponse { token }))
}