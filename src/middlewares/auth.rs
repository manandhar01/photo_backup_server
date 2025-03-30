use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::auth::jwt::validate_token;

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    match token {
        Some(token) => match validate_token(token) {
            Ok(_) => Ok(next.run(req).await),
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        },
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
