use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::user::dtos::user::UserResponse;
use crate::user::services::user::UserService;
use crate::{app::AppState, common::traits::model_ops::ModelOps};

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

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<(), (StatusCode, String)> {
    let mut user = UserService::find_user_by_id(&state.db, user_id)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

    user.soft_delete(&state.db).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not delete user".to_string(),
        )
    })?;

    Ok(())
}
