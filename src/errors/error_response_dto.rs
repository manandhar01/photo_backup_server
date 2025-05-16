use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;

use super::app_error::AppError;

#[derive(Debug, Serialize)]
pub struct ErrorResponseDto {
    pub message: String,
    pub status: u16,
}

impl ErrorResponseDto {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status: status.as_u16(),
            message: message.into(),
        }
    }
}

impl IntoResponse for ErrorResponseDto {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(&self);

        (status, body).into_response()
    }
}

impl From<AppError> for ErrorResponseDto {
    fn from(error: AppError) -> Self {
        match error {
            AppError::BadRequest(message) => {
                ErrorResponseDto::new(StatusCode::BAD_REQUEST, message)
            }
            AppError::Unauthorized(message) => {
                ErrorResponseDto::new(StatusCode::UNAUTHORIZED, message)
            }
            AppError::NotFound(message) => ErrorResponseDto::new(StatusCode::NOT_FOUND, message),
            AppError::InternalServerError(message) => {
                ErrorResponseDto::new(StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::EndOfFile => ErrorResponseDto::new(StatusCode::NO_CONTENT, "".to_string()),
        }
    }
}
