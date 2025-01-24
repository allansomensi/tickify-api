use crate::models::jwt::Claims;
use axum::http::StatusCode;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn generate_jwt(username: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration as usize,
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
