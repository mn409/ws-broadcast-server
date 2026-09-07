use axum::{
    extract::{State, Request},
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::{
    state::AppState,
    models::jwt::verify_token,
};

pub async fn auth_middleware(
    State(_state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {

    let auth_header = match req.headers().get(AUTHORIZATION) {
        Some(h) => h.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let claims = match verify_token(token) {
        Ok(c) => c,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}