use crate::database::AppState;
use crate::models::{
    user::{CreateUserPayload, UpdateUserPayload},
    DeletePayload,
};
use crate::utils::hashing::encrypt_password;
use crate::validations::{existence::user_exists, uniqueness::is_user_unique};
use crate::{errors::api_error::ApiError, models::user::User};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use std::sync::Arc;
use tracing::{debug, error, info};
use uuid::Uuid;
use validator::Validate;

/// Retrieves the total count of users.
///
/// This endpoint counts all users stored in the database and returns the count as an integer.
/// If no users are found, 0 is returned.
#[utoipa::path(
    get,
    path = "/api/v1/users/count",
    tags = ["Users"],
    summary = "Get the total count of users.",
    description = "This endpoint retrieves the total number of users stored in the database.",
    responses(
        (status = 200, description = "User count retrieved successfully.", body = i32),
        (status = 500, description = "An error occurred while retrieving the user count.")
    )
)]
pub async fn count_users(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    debug!("Received request to retrieve user count.");

    match User::count(&state).await {
        Ok(count) => {
            info!("Successfully retrieved user count: {count}");
            Ok(Json(count))
        }
        Err(e) => {
            error!("Failed to retrieve user count: {e}");
            Err(ApiError::from(e))
        }
    }
}

/// Retrieves a list of all users.
///
/// This endpoint fetches all users stored in the database.
/// If there are no users, returns an empty array.
#[utoipa::path(
    get,
    path = "/api/v1/users",
    tags = ["Users"],
    summary = "List all users.",
    description = "Fetches all users stored in the database. If there are no users, returns an empty array.",
    responses(
        (status = 200, description = "Users retrieved successfully.", body = Vec<User>),
        (status = 404, description = "No users found in the database."),
        (status = 500, description = "An error occurred while retrieving the users.")
    )
)]
pub async fn find_all_users(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    debug!("Received request to retrieve all users.");

    match User::find_all(&state).await {
        Ok(users) => {
            info!("Users listed successfully.");
            Ok(Json(users))
        }
        Err(e) => {
            error!("Error retrieving all users: {e}");
            Err(ApiError::from(e))
        }
    }
}

/// Retrieves a specific user by its ID.
///
/// This endpoint searches for a user with the specified ID.
/// If the user is found, it returns the user details.
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tags = ["Users"],
    summary = "Get a specific user by ID.",
    description = "This endpoint retrieves a user's details from the database using its ID. Returns the user if found, or a 404 status if not found.",
    params(
        ("id", description = "The unique identifier of the user to retrieve.", example = Uuid::new_v4)
    ),
    responses(
        (status = 200, description = "User retrieved successfully.", body = User),
        (status = 404, description = "No user found with the specified ID."),
        (status = 500, description = "An error occurred while retrieving the user.")
    )
)]
pub async fn find_user_by_id(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    debug!("Received request to retrieve user with id: {id}");

    match User::find_by_id(&state, id).await {
        Ok(Some(user)) => {
            info!("User found: {id}");
            Ok(Json(user))
        }
        Ok(None) => {
            error!("No user found with id: {id}");
            Err(ApiError::NotFound)
        }
        Err(e) => {
            error!("Error retrieving user with id {id}: {e}");
            Err(ApiError::from(e))
        }
    }
}

/// Create a new user.
///
/// This endpoint creates a new user by providing its details.
/// Validates the user's name for length and emptiness, checks for duplicates,
/// and inserts the new user into the database if all validations pass.
#[utoipa::path(
    post,
    path = "/api/v1/users",
    tags = ["Users"],
    summary = "Create a new user.",
    description = "This endpoint creates a new user in the database with the provided details.",
    request_body = CreateUserPayload,
    responses(
        (status = 201, description = "User created successfully.", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long."),
        (status = 409, description = "Conflict: User with the same name already exists."),
        (status = 500, description = "An error occurred while creating the user.")
    )
)]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    payload.validate()?;
    is_user_unique(state.clone(), payload.username.clone()).await?;

    let new_user = User::new(
        &payload.username,
        payload.email,
        encrypt_password(&payload.password)
            .expect("Error encrypting password")
            .as_str(),
        payload.first_name,
        payload.last_name,
    );

    // Creates the user.
    sqlx::query(r#"INSERT INTO users (id, username, email, password_hash, first_name, last_name, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
        .bind(new_user.id)
        .bind(&new_user.username)
        .bind(&new_user.email)
        .bind(&new_user.password_hash)
        .bind(&new_user.first_name)
        .bind(&new_user.last_name)
        .bind(new_user.created_at)
        .bind(new_user.updated_at)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error creating user: {e}");
            ApiError::DatabaseError(e)
        })?;
    info!("User created! ID: {}", &new_user.id);
    Ok((StatusCode::CREATED, Json(new_user.id)))
}

// /// Updates an existing user.
// ///
// /// This endpoint updates the details of an existing user.
// /// It accepts the user ID and the new details for the user.
// /// The endpoint validates the new name to ensure it is not empty,
// /// does not conflict with an existing user's name, and meets length requirements.
// /// If the user is successfully updated, it returns the UUID of the updated user.
#[utoipa::path(
    put,
    path = "/api/v1/users",
    tags = ["Users"],
    summary = "Update an existing user.",
    description = "This endpoint updates the details of an existing user in the database.",
    request_body = UpdateUserPayload,
    responses(
        (status = 200, description = "User updated successfully.", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long."),
        (status = 404, description = "User ID not found."),
        (status = 409, description = "Conflict: User with the same name already exists."),
        (status = 500, description = "An error occurred while updating the user.")
    )
)]
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    payload.validate()?;
    user_exists(state.clone(), payload.id).await?;

    let user_id = payload.id;
    let new_username = payload.username;
    let new_email = payload.email;
    let new_password = payload.password;
    let new_first_name = payload.first_name;
    let new_last_name = payload.last_name;

    let mut updated = false;

    // Update `username` if provided.
    if let Some(username) = new_username {
        sqlx::query(r#"UPDATE users SET username = $1 WHERE id = $2;"#)
            .bind(username)
            .bind(user_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating username: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update `email` if provided.
    if let Some(email) = new_email {
        sqlx::query(r#"UPDATE users SET email = $1 WHERE id = $2;"#)
            .bind(email)
            .bind(user_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating email: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Encrypt and update the `password` if provided
    if let Some(password) = new_password {
        let encrypted_password = encrypt_password(&password)?;

        sqlx::query(r#"UPDATE users SET password_hash = $1 WHERE id = $2;"#)
            .bind(&encrypted_password)
            .bind(user_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating password: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update `first_name` if provided
    if let Some(first_name) = new_first_name {
        sqlx::query(r#"UPDATE users SET first_name = $1 WHERE id = $2;"#)
            .bind(first_name)
            .bind(user_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating first_name: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update `last_name` if provided
    if let Some(last_name) = new_last_name {
        sqlx::query(r#"UPDATE users SET last_name = $1 WHERE id = $2;"#)
            .bind(last_name)
            .bind(user_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating last_name: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Updates `updated_at` field.
    if updated {
        sqlx::query(r#"UPDATE users SET updated_at = $1 WHERE id = $2;"#)
            .bind(Utc::now().naive_utc())
            .bind(user_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating last_name: {e}");
                ApiError::DatabaseError(e)
            })?;
    } else {
        error!(
            "No updates were made for the provided user ID: {}",
            &user_id
        );
        return Err(ApiError::NotModified);
    }

    info!("User updated! ID: {}", &user_id);
    Ok((StatusCode::OK, Json(user_id)).into_response())
}

/// Deletes an existing user.
///
/// This endpoint allows users to delete a specific user by its ID.
/// It checks if the user exists before attempting to delete it.
/// If the user is successfully deleted, a confirmation message is returned.
#[utoipa::path(
    delete,
     path = "/api/v1/users",
     tags = ["Users"],
     summary = "Delete an existing user.",
     description = "This endpoint deletes a specific user from the database using its ID.",
     request_body = DeletePayload,
     responses(
         (status = 200, description = "User deleted successfully"),
         (status = 404, description = "User ID not found"),
         (status = 500, description = "An error occurred while deleting the user")
     )
 )]
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeletePayload>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    user_exists(state.clone(), payload.id).await?;

    // Delete the user
    sqlx::query(r#"DELETE FROM users WHERE id = $1;"#)
        .bind(payload.id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error deleting user: {}", e);
            ApiError::DatabaseError(e)
        })?;

    info!("User deleted! ID: {}", &payload.id);
    Ok((StatusCode::OK, Json("User deleted!")))
}
