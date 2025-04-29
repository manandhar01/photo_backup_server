use axum::{extract::State, Json};
use std::sync::Arc;

use crate::app::AppState;
use crate::auth::{
    dtos::{
        login::{LoginRequest, LoginResponse},
        register::RegisterRequest,
        verify_token_response::VerifyTokenResponse,
    },
    services::auth::AuthService,
};
use crate::errors::app_error::AppError;
use crate::user::{dtos::user::UserResponse, services::user::UserService};
use crate::utility::hash::hash_password;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let hashed_password = hash_password(&payload.password)?;

    let user = UserService::create_user(
        &state.db,
        &payload.email,
        &payload.username,
        &hashed_password,
    )
    .await
    .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?;

    Ok(Json(user.into()))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = UserService::find_user_by_email(&state.db, &payload.email)
        .await
        .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?;

    match user {
        Some(user) => {
            if !user.verify_password(&payload.password) {
                return Err(AppError::Unauthorized("Invalid credentials".to_string()));
            }

            let token = AuthService::generate_token(user.uuid)
                .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?;

            Ok(Json(LoginResponse { token }))
        }
        None => Err(AppError::Unauthorized("Invalid credentials".to_string())),
    }
}

pub async fn verify() -> Json<VerifyTokenResponse> {
    Json(VerifyTokenResponse { valid: true })
}
