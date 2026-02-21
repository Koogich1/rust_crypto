use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn auth_middleware(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let expected_token = env::var("API_SECRET_TOKEN").ok();

    let Some(expected_token) = expected_token else {
        tracing::warn!("⚠️ API_SECRET_TOKEN not set, skipping auth");
        return Ok(next.run(request).await);
    };

    let is_valid = auth_header
        .map(|h| h.starts_with("Bearer ") && h[7..] == expected_token)
        .unwrap_or(false);

    if !is_valid {
        tracing::warn!("🚫 Unauthorized request: {:?}", request.uri());
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
