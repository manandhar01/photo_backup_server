use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;

use super::app_error::AppError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: u16,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status: status.as_u16(),
            message: message.into(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(&self);

        (status, body).into_response()
    }
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        match error {
            AppError::BadRequest(message) => ErrorResponse::new(StatusCode::BAD_REQUEST, message),
            AppError::Unauthorized(message) => {
                ErrorResponse::new(StatusCode::UNAUTHORIZED, message)
            }
            AppError::NotFound(message) => ErrorResponse::new(StatusCode::NOT_FOUND, message),
            AppError::InternalServerError(message) => {
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::EndOfFile => ErrorResponse::new(StatusCode::NO_CONTENT, "".to_string()),
        }
    }
}
