use crate::{
    database::AppState,
    errors::api_error::ApiError,
    models::auth::{LoginPayload, VerifyTokenPayload},
    utils::{
        hashing::verify_password,
        jwt::{generate_jwt, validate_jwt},
    },
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::{error, info};

/// Returns a JWT if the credentials passed are valid.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tags = ["Auth"],
    summary = "Returns a JTW.",
    description = "Returns a JWT if the credentials passed are valid.",
    request_body = LoginPayload,
    responses(
        (status = 201, description = "Logged in successfully."),
        (status = 401, description = "Incorrect password, unauthorized."),
        (status = 404, description = "User not found."),
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let password_hash: Option<String> =
        sqlx::query_scalar(r#"SELECT password_hash FROM users WHERE username = $1;"#)
            .bind(&payload.username)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| {
                error!("Error retrieving password_hash: {e}");
                ApiError::NotFound
            })?;

    let password_hash = match password_hash {
        Some(hash) => hash,
        None => return Err(ApiError::NotFound),
    };

    let is_password_correct = verify_password(&payload.password, &password_hash)?;

    if !is_password_correct {
        error!("Incorrect password for user: {}", payload.username);
        return Err(ApiError::Unauthorized);
    }

    let token = generate_jwt(&payload.username);

    info!("Login successful for user: {}", payload.username);

    Ok((StatusCode::OK, Json(token)))
}

/// Checks if a JWT is valid.
#[utoipa::path(
    post,
    path = "/api/v1/auth/verify",
    tags = ["Auth"],
    summary = "Verify JWT.",
    description = "Checks if a JWT is valid.",
    request_body = VerifyTokenPayload,
    responses(
        (status = 201, description = "JWT provided is valid."),
    )
)]
pub async fn verify(
    Json(payload): Json<VerifyTokenPayload>,
) -> Result<impl IntoResponse, ApiError> {
    validate_jwt(&payload.token)?;
    info!("Successful verified token");

    Ok((StatusCode::OK, Json("Token is valid!")))
}
