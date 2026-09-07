use axum::{
    extract::{State, Query},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::{collections::HashMap, sync::Arc};

use crate::{
    state::AppState,
    models::jwt::verify_token,
};

pub async fn auth_middleware(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
    mut request: axum::http::Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = match params.get("token") {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let user_id = match verify_token(token) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}