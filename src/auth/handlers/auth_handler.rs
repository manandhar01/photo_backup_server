use axum::{
    extract::{ConnectInfo, State},
    Extension, Json,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use chrono::{DateTime, TimeZone, Utc};
use core::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, warn};

use crate::app::AppState;
use crate::auth::{
    dtos::{
        LoginActivityDto, LoginRequestDto, LoginResponseDto, RefreshTokenPayloadDto,
        RegisterRequestDto, VerifyTokenResponseDto,
    },
    services::{LoginActivityService, RefreshTokenService},
};
use crate::errors::app_error::AppError;
use crate::user::{dtos::UserResponseDto, models::UserModel, services::UserService};
use crate::utility::hash::hash_password;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequestDto>,
) -> Result<Json<UserResponseDto>, AppError> {
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
    Json(payload): Json<LoginRequestDto>,
) -> Result<Json<LoginResponseDto>, AppError> {
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

            let response = match RefreshTokenService::generate_token_pair(
                &state.db, &user, None, None,
            )
            .await
            {
                Ok(response) => response,
                Err(e) => {
                    match LoginActivityService::create_log(&state.db, activity).await {
                        Ok(_activity) => {}
                        Err(e) => eprintln!("Failed to save login activity: {}", e),
                    };

                    warn!("Login failed: email={}, reason={:?}", &payload.email, e);

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

            Ok(response)
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

pub async fn refresh_tokens(
    State(state): State<Arc<AppState>>,
    Extension(refresh_token_payload): Extension<RefreshTokenPayloadDto>,
    Extension(user): Extension<UserModel>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Result<Json<LoginResponseDto>, AppError> {
    let mut activity = LoginActivityDto {
        user_id: None,
        email: user.email.clone(),
        success: false,
        ip_address: Some(addr.ip().to_string()),
        user_agent: Some(user_agent.to_string()),
    };

    let exp = refresh_token_payload.exp as i64;

    let expires_at: Option<DateTime<Utc>> = Utc.timestamp_opt(exp, 0).single();

    let response = match RefreshTokenService::generate_token_pair(
        &state.db,
        &user,
        Some(refresh_token_payload.token),
        expires_at,
    )
    .await
    {
        Ok(response) => response,
        Err(e) => {
            match LoginActivityService::create_log(&state.db, activity).await {
                Ok(_activity) => {}
                Err(e) => eprintln!("Failed to save login activity: {}", e),
            };

            warn!("Login failed: email={}, reason={:?}", &user.email, e);

            return Err(AppError::InternalServerError("Something went wrong".into()));
        }
    };

    activity.user_id = Some(user.id);
    activity.success = true;
    match LoginActivityService::create_log(&state.db, activity).await {
        Ok(_activity) => {}
        Err(e) => eprintln!("Failed to save login activity: {}", e),
    };

    info!(
        "Login successful: user_id={}, email={}",
        &user.id, &user.email
    );

    Ok(response)
}

pub async fn verify() -> Json<VerifyTokenResponseDto> {
    Json(VerifyTokenResponseDto { valid: true })
}
