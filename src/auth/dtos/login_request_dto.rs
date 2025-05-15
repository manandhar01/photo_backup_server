use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequestDto {
    pub email: String,
    pub password: String,
}
