use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequestDto {
    pub email: String,
    pub username: String,
    pub password: String,
}
