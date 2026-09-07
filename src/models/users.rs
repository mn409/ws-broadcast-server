use serde::Deserialize;
use axum::response::IntoResponse;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

pub async fn signup(
    Json(payload): Json<SignupRequest>,
    State(state): State<AppState>,
) -> impl IntoResponse {

}