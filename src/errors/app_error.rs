use axum::response::{IntoResponse, Response};

use crate::errors::error_response_dto::ErrorResponseDto;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    InternalServerError(String),
    EndOfFile,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        ErrorResponseDto::from(self).into_response()
    }
}
