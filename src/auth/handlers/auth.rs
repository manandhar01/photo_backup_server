use axum::{
    extract::{ConnectInfo, State},
    Json,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use core::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, warn};

use crate::app::AppState;
use crate::auth::{
    dtos::{
        login::{LoginRequest, LoginResponse},
        login_activity_dto::LoginActivityDto,
        register::RegisterRequest,
        verify_token_response::VerifyTokenResponse,
    },
    services::{auth::AuthService, login_activity::LoginActivityService},
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
    .map_err(|_| AppError::InternalServerError("Something went wrong".into()))?;

    Ok(Json(user.into()))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let mut activity = LoginActivityDto {
        user_id: None,
        email: payload.email.clone(),
        success: false,
        ip_address: Some(addr.ip().to_string()),
        user_agent: Some(user_agent.to_string()),
    };

    let user = match UserService::find_user_by_email(&state.db, &payload.email).await {
        Ok(user) => user,
        Err(e) => {
            match LoginActivityService::create_log(&state.db, activity).await {
                Ok(_activity) => {}
                Err(e) => eprintln!("Failed to save login activity: {}", e),
            };

            warn!("Login failed: email={}, reason={}", &payload.email, e);

            return Err(AppError::Unauthorized("Invalid credentials".into()))?;
        }
    };

    match user {
        Some(user) => {
            activity.user_id = Some(user.id);

            if !user.verify_password(&payload.password) {
                match LoginActivityService::create_log(&state.db, activity).await {
                    Ok(_activity) => {}
                    Err(e) => eprintln!("Failed to save login activity: {}", e),
                };

                println!("here");

                warn!(
                    "Login failed: email={}, reason={}",
                    &payload.email, "wrong password"
                );

                return Err(AppError::Unauthorized("Invalid credentials".into()));
            }

            let token = match AuthService::generate_token(user.uuid) {
                Ok(token) => token,
                Err(e) => {
                    match LoginActivityService::create_log(&state.db, activity).await {
                        Ok(_activity) => {}
                        Err(e) => eprintln!("Failed to save login activity: {}", e),
                    };

                    warn!("Login failed: email={}, reason={}", &payload.email, e);

                    return Err(AppError::InternalServerError("Something went wrong".into()));
                }
            };

            activity.success = true;
            match LoginActivityService::create_log(&state.db, activity).await {
                Ok(_activity) => {}
                Err(e) => eprintln!("Failed to save login activity: {}", e),
            };

            info!(
                "Login successful: user_id={}, email={}",
                &user.id, &user.email
            );

            Ok(Json(LoginResponse { token }))
        }
        None => {
            match LoginActivityService::create_log(&state.db, activity).await {
                Ok(_activity) => {}
                Err(e) => eprintln!("Failed to save login activity: {}", e),
            };

            warn!(
                "Login failed: email={}, reason={}",
                &payload.email, "user not found"
            );

            Err(AppError::Unauthorized("Invalid credentials".into()))
        }
    }
}

pub async fn verify() -> Json<VerifyTokenResponse> {
    Json(VerifyTokenResponse { valid: true })
}
