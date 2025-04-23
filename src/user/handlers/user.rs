use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::app::AppState;
use crate::user::dtos::user::UserResponse;
use crate::user::services::user::UserService;

pub async fn get_user_by_uuid(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = UserService::find_user_by_uuid(&state.db, uuid)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_string(),
            )
        })?;

    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err((StatusCode::BAD_REQUEST, "User not found".to_string())),
    }
}

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = UserService::find_user_by_id(&state.db, user_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_string(),
            )
        })?;

    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err((StatusCode::BAD_REQUEST, "User not found".to_string())),
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = UserService::delete_user(&state.db, user_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_string(),
            )
        })?;

    match user {
        Some(user) => Ok(Json(user.into())),
        None => Err((StatusCode::BAD_REQUEST, "User not found".to_string())),
    }
}
