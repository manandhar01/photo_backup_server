use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::auth::services::auth::AuthService;

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    match token {
        Some(token) => match AuthService::validate_token(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);

                Ok(next.run(req).await)
            }
            Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())),
        },
        None => Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())),
    }
}
