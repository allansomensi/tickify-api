use crate::models::jwt::Claims;
use axum::http::StatusCode;
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use std::env;

pub fn generate_jwt(username: &str) -> String {
    let now = Utc::now();
    let expire: TimeDelta = Duration::seconds(
        env::var("JWT_EXPIRATION_TIME")
            .expect("Error reading JWT_EXPIRATION_TIME")
            .parse()
            .expect("Invalid JWT_EXPIRATION_TIME value"),
    );
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claims = Claims {
        iat,
        sub: username.to_string(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("Error reading JWT_SECRET")
                .as_bytes(),
        ),
    )
    .expect("Error creating token");

    token
}

pub fn validate_jwt(token: &str) -> Result<(), jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let _: TokenData<Claims> = decode(
        token,
        &DecodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("Error reading JWT_SECRET env var")
                .as_bytes(),
        ),
        &validation,
    )?;
    Ok(())
}

pub fn decode_jwt(token: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = env::var("JWT_SECRET").expect("Error reading JWT_SECRET env var");
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}
