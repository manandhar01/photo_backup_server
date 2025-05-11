use dotenvy::dotenv;
use std::env;

pub fn get_jwt_secret() -> String {
    dotenv().ok();

    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn get_access_token_expiry() -> i64 {
    dotenv().ok();

    let default_value = 60 * 60;

    env::var("JWT_VALIDITY")
        .unwrap_or_else(|_| format!("{}", default_value))
        .parse::<i64>()
        .unwrap_or(default_value)
}

pub fn get_refresh_token_expiry() -> i64 {
    dotenv().ok();

    let default_value = 2 * 24 * 60 * 60;

    env::var("JWT_VALIDITY")
        .unwrap_or_else(|_| format!("{}", default_value))
        .parse::<i64>()
        .unwrap_or(default_value)
}
