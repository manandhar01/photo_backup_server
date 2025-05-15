#[derive(Clone)]
pub struct RefreshTokenPayloadDto {
    pub exp: usize,
    pub token: String,
}
