use crate::{database::AppState, errors::api_error::ApiError, utils::hashing::encrypt_password};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(ToSchema, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateUserPayload {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 chars."
    ))]
    pub username: String,
    #[validate(email(message = "Invalid email"))]
    pub email: Option<String>,
    #[validate(length(
        min = 8,
        max = 100,
        message = "Password must be between 8 and 100 chars."
    ))]
    pub password: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "First name must be between 3 and 20 chars."
    ))]
    pub first_name: Option<String>,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Last name must be between 3 and 20 chars."
    ))]
    pub last_name: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateUserPayload {
    pub id: Uuid,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 chars."
    ))]
    pub username: Option<String>,
    #[validate(email(message = "Invalid email"))]
    pub email: Option<String>,
    #[validate(length(
        min = 8,
        max = 100,
        message = "Password must be between 8 and 100 chars."
    ))]
    pub password: Option<String>,
    #[validate(length(
        min = 3,
        max = 20,
        message = "First name must be between 3 and 20 chars."
    ))]
    pub first_name: Option<String>,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Last name must be between 3 and 20 chars."
    ))]
    pub last_name: Option<String>,
}

impl User {
    pub fn new(
        username: &str,
        email: Option<String>,
        password: &str,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email,
            password_hash: password.to_string(),
            first_name,
            last_name,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }

    pub async fn count(state: &AppState) -> Result<i64, ApiError> {
        debug!("Attempting to count users from the database...");

        let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM users;"#)
            .fetch_one(&state.db)
            .await?;

        Ok(count)
    }

    pub async fn find_all(state: &AppState) -> Result<Vec<Self>, ApiError> {
        debug!("Attempting to retrieve all users from the database...");

        let users: Vec<Self> = sqlx::query_as(r#"SELECT * FROM users;"#)
            .fetch_all(&state.db)
            .await?;

        Ok(users)
    }

    pub async fn find_by_id(state: &AppState, id: Uuid) -> Result<Option<Self>, ApiError> {
        debug!("Attempting to retrieve user with id: {id}");

        let user: Option<Self> = sqlx::query_as(r#"SELECT * FROM users WHERE id = $1;"#)
            .bind(id)
            .fetch_optional(&state.db)
            .await?;

        Ok(user)
    }

    pub async fn create(state: &AppState, payload: &CreateUserPayload) -> Result<Self, ApiError> {
        debug!(
            "Attempting to create user with username: {}",
            payload.username
        );

        let new_user = Self::new(
            &payload.username,
            payload.email.clone(),
            encrypt_password(&payload.password)?.as_str(),
            payload.first_name.clone(),
            payload.last_name.clone(),
        );

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
    .await?;

        Ok(new_user)
    }

    pub async fn update(state: &AppState, payload: &UpdateUserPayload) -> Result<Uuid, ApiError> {
        debug!("Attempting to update user with ID: {}", payload.id);

        let user_id = payload.id;
        let new_username = &payload.username;
        let new_email = &payload.email;
        let new_password = &payload.password;
        let new_first_name = &payload.first_name;
        let new_last_name = &payload.last_name;

        let mut updated = false;

        // Update `username` if provided.
        if let Some(username) = new_username {
            sqlx::query(r#"UPDATE users SET username = $1 WHERE id = $2;"#)
                .bind(username)
                .bind(user_id)
                .execute(&state.db)
                .await?;
            updated = true;
        }

        // Update `email` if provided.
        if let Some(email) = new_email {
            sqlx::query(r#"UPDATE users SET email = $1 WHERE id = $2;"#)
                .bind(email)
                .bind(user_id)
                .execute(&state.db)
                .await?;
            updated = true;
        }

        // Encrypt and update the `password` if provided
        if let Some(password) = new_password {
            let encrypted_password = encrypt_password(&password)?;

            sqlx::query(r#"UPDATE users SET password_hash = $1 WHERE id = $2;"#)
                .bind(&encrypted_password)
                .bind(user_id)
                .execute(&state.db)
                .await?;
            updated = true;
        }

        // Update `first_name` if provided
        if let Some(first_name) = new_first_name {
            sqlx::query(r#"UPDATE users SET first_name = $1 WHERE id = $2;"#)
                .bind(first_name)
                .bind(user_id)
                .execute(&state.db)
                .await?;
            updated = true;
        }

        // Update `last_name` if provided
        if let Some(last_name) = new_last_name {
            sqlx::query(r#"UPDATE users SET last_name = $1 WHERE id = $2;"#)
                .bind(last_name)
                .bind(user_id)
                .execute(&state.db)
                .await?;
            updated = true;
        }

        // Updates `updated_at` field.
        if updated {
            sqlx::query(r#"UPDATE users SET updated_at = $1 WHERE id = $2;"#)
                .bind(Utc::now().naive_utc())
                .bind(user_id)
                .execute(&state.db)
                .await?;
        } else {
            return Err(ApiError::NotModified);
        }

        Ok(user_id)
    }
}
