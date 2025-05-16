use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::app::AppState;
use crate::auth::services::AuthService;
use crate::errors::app_error::AppError;
use crate::user::{dtos::UserResponseDto, services::user_service::UserService};

pub async fn get_user_by_uuid(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<UserResponseDto>, AppError> {
    let user = UserService::find_user_by_uuid(&state.db, uuid)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?;

    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err(AppError::NotFound("User not found".to_string())),
    }
}

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponseDto>, AppError> {
    let user = UserService::find_user_by_id(&state.db, user_id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?;

    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err(AppError::NotFound("User not found".to_string())),
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponseDto>, AppError> {
    let user = UserService::delete_user(&state.db, user_id)
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?;

    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err(AppError::NotFound("User not found".to_string())),
    }
}

pub async fn get_self(
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserResponseDto>, AppError> {
    match AuthService::id() {
        Some(user_id) => {
            let user = UserService::find_user_by_id(&state.db, user_id)
                .await
                .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))?;

            match user {
                Some(user) => Ok(Json(user.into())),
                None => Err(AppError::NotFound("User not found".to_string())),
            }
        }

        None => Err(AppError::Unauthorized("Invalid credentials".to_string())),
    }
}
