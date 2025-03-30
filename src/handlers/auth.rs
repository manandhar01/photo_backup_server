use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::jwt::generate_token;
use crate::dtos::{
    login::{LoginRequest, LoginResponse},
    register::RegisterRequest,
    user::UserResponse,
};
use crate::services::user::UserService;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = UserService::create_user(
        &state.db,
        &payload.email,
        &payload.username,
        &payload.password,
    )
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(user.into()))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let user = UserService::find_user_by_email(&state.db, &payload.email)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    if !user.verify_password(&payload.password) {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let token = generate_token(&user.uuid.to_string()).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to generate token".to_string(),
        )
    })?;

    Ok(Json(LoginResponse { token }))
}
