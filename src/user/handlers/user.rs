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
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user.into()))
}

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = UserService::find_user_by_id(&state.db, user_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user.into()))
}
