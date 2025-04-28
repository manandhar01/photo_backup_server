use bcrypt::hash;

use crate::errors::app_error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, 12)
        .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))
}
