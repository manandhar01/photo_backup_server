use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::auth::dtos::claims::Claims;
use crate::config::{get_jwt_expiry, get_jwt_secret};

pub struct AuthService;

impl AuthService {
    pub fn generate_token(uuid: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::seconds(get_jwt_expiry()))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: uuid,
            exp: expiration,
        };

        let secret = get_jwt_secret();
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = get_jwt_secret();
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }
}
