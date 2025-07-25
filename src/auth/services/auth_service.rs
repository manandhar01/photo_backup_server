use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::auth::dtos::ClaimsDto;
use crate::config::{get_access_token_expiry, get_jwt_secret, get_refresh_token_expiry};
use crate::user::models::UserModel;

tokio::task_local! {
    pub static CURRENT_USER: UserModel;
}

pub struct AuthService {}

impl AuthService {
    pub fn generate_access_token(id: i32) -> Result<String, jsonwebtoken::errors::Error> {
        let validity = get_access_token_expiry();

        let expiration = Utc::now()
            .checked_add_signed(Duration::seconds(validity))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = ClaimsDto {
            sub: id,
            exp: expiration,
            refresh: false,
        };

        let secret = get_jwt_secret();
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn generate_refresh_token(id: i32) -> Result<String, jsonwebtoken::errors::Error> {
        let validity = get_refresh_token_expiry();

        let expiration = Utc::now()
            .checked_add_signed(Duration::seconds(validity))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = ClaimsDto {
            sub: id,
            exp: expiration,
            refresh: true,
        };

        let secret = get_jwt_secret();
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn validate_token(token: &str) -> Result<ClaimsDto, jsonwebtoken::errors::Error> {
        let secret = get_jwt_secret();
        decode::<ClaimsDto>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }

    pub fn user() -> Option<UserModel> {
        CURRENT_USER.try_with(|u| u.clone()).ok()
    }

    // pub fn check() -> bool {
    //     Self::user().is_some()
    // }

    pub fn id() -> Option<i32> {
        Self::user().map(|u| u.id)
    }

    pub async fn login<R, F>(user: UserModel, fut: F) -> R
    where
        F: std::future::Future<Output = R>,
    {
        CURRENT_USER.scope(user, fut).await
    }
}
