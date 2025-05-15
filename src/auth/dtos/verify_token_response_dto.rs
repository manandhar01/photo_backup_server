use serde::Serialize;

#[derive(Serialize)]
pub struct VerifyTokenResponseDto {
    pub valid: bool,
}
