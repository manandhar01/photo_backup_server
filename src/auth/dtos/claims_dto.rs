use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaimsDto {
    pub sub: i32,
    pub exp: usize,
    pub refresh: bool,
}
