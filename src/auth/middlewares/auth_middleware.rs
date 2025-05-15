use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::services::auth_service::AuthService;
use crate::errors::app_error::AppError;
use crate::user::services::user_service::UserService;

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Response {
    if let Some(token) = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
    {
        if let Ok(claims) = AuthService::validate_token(token) {
            if claims.refresh {
                return AppError::Unauthorized("Invalid credentials".into()).into_response();
            }

            if let Ok(Some(user)) = UserService::find_user_by_id(&state.db, claims.sub).await {
                req.extensions_mut().insert(user.clone());

                return AuthService::login(user, async { next.run(req).await }).await;
            }
        }
    }

    AppError::Unauthorized("Invalid credentials".into()).into_response()
}
