use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponseDto {
    pub access_token: String,
    pub refresh_token: String,
}
