use dotenvy::dotenv;
use std::env;

pub fn get_jwt_secret() -> String {
    dotenv().ok();

    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn get_jwt_expiry() -> i64 {
    dotenv().ok();

    env::var("JWT_EXPIRY")
        .unwrap_or_else(|_| "3600".to_string())
        .parse()
        .unwrap()
}
