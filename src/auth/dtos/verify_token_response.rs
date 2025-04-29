use serde::Serialize;

#[derive(Serialize)]
pub struct VerifyTokenResponse {
    pub valid: bool,
}
