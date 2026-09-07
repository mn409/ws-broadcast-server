use crate::state::AppState;
use axum::{
    Json as AxumJson,
    extract::{Json, State},
    response::IntoResponse,
};
use serde::Deserialize;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use sqlx::Row;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn signup(
    Json(payload): Json<SignupRequest>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let username = payload.username;
    let password = payload.password;

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => {
            return AxumJson(serde_json::json!({
                "error": "Failed to hash password"
            }));
        }
    };

    let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
        .bind(username.clone())
        .bind(password_hash)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => AxumJson(serde_json::json!({
            "message": "User created successfully"
        })),
        Err(_) => AxumJson(serde_json::json!({
            "error": "Failed to create user (maybe username exists)"
        })),
    }
}

pub async fn login(
    Json(payload): Json<LoginRequest>,
    State(state): State<AppState>,
) -> impl IntoResponse {

    let username = payload.username;
    let password = payload.password;

    let result = sqlx::query(
        "SELECT id, password_hash FROM users WHERE username = $1"
    )
    .bind(username.clone())
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {

            let user_id: i32 = row.get("id");
            let password_hash: String = row.get("password_hash");

            let parsed_hash = match PasswordHash::new(&password_hash) {
                Ok(hash) => hash,
                Err(_) => {
                    return AxumJson(serde_json::json!({
                        "error": "Server error"
                    }));
                }
            };

            match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
                Ok(_) => {

                    use crate::models::jwt::generate_token;

                    let token = match generate_token(&user_id.to_string()) {
                        Ok(t) => t,
                        Err(_) => {
                            return AxumJson(serde_json::json!({
                                "error": "Failed to generate token"
                            }));
                        }
                    };

                    AxumJson(serde_json::json!({
                        "message": "Login successful",
                        "token": token
                    }))
                }

                Err(_) => AxumJson(serde_json::json!({
                    "error": "Invalid username or password"
                })),
            }
        }

        Err(_) => AxumJson(serde_json::json!({
            "error": "Invalid username or password"
        })),
    }
}