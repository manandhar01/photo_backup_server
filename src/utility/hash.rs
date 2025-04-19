use axum::http::StatusCode;
use bcrypt::hash;

pub fn hash_password(password: &str) -> Result<String, (StatusCode, String)> {
    match hash(password, 12) {
        Ok(hash) => Ok(hash),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )),
    }
}
