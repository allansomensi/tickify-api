use crate::{
    database::AppState,
    errors::api_error::ApiError,
    models::{
        auth::{LoginPayload, VerifyTokenPayload},
        user::{CreateUserPayload, User},
    },
    utils::{
        hashing::verify_password,
        jwt::{generate_jwt, validate_jwt},
    },
    validations::uniqueness::is_user_unique,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::{debug, error, info};
use validator::Validate;

/// Returns a JWT if the credentials passed are valid.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tags = ["Auth"],
    summary = "Returns a JTW.",
    description = "If the credentials are correct, a JWT is returned.",
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
            .await?;

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

/// Register a new user.
///
/// This endpoint registers a new user in the database.
/// It is essentially the same as the `create_user` handler, but does not require authentication.
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tags = ["Auth"],
    summary = "Register a new user.",
    description = "This endpoint register a new user in the database with the provided details.",
    request_body = CreateUserPayload,
    responses(
        (status = 201, description = "User registered successfully.", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long."),
        (status = 409, description = "Conflict: User with the same name already exists."),
        (status = 500, description = "An error occurred while creating the user.")
    )
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<impl IntoResponse, ApiError> {
    debug!(
        "Received request to register user with username: {}",
        payload.username
    );

    // Validations
    payload.validate()?;
    is_user_unique(&state, &payload.username).await?;

    match User::create(&state, &payload).await {
        Ok(new_user) => {
            info!("User created! ID: {}", &new_user.id);
            Ok((StatusCode::CREATED, Json(new_user.id)))
        }
        Err(e) => {
            error!(
                "Error creating user with username {}: {e}",
                payload.username
            );
            Err(ApiError::from(e))
        }
    }
}

/// Checks if a JWT is valid.
#[utoipa::path(
    post,
    path = "/api/v1/auth/verify",
    tags = ["Auth"],
    summary = "Verify JWT.",
    description = "Verifies if a JWT is still valid.",
    request_body = VerifyTokenPayload,
    responses(
        (status = 201, description = "Token is valid!"),
    )
)]
pub async fn verify(
    Json(payload): Json<VerifyTokenPayload>,
) -> Result<impl IntoResponse, ApiError> {
    validate_jwt(&payload.token)?;
    info!("Successful verified token");

    Ok((StatusCode::OK, Json("Token is valid!")))
}
