use super::DeletePayload;
use crate::{
    database::{
        repositories::user_repository::{UserRepository, UserRepositoryImpl},
        AppState,
    },
    errors::api_error::ApiError,
};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
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
        Ok(UserRepositoryImpl::count(state).await?)
    }

    pub async fn find_all(state: &AppState) -> Result<Vec<Self>, ApiError> {
        Ok(UserRepositoryImpl::find_all(state).await?)
    }

    pub async fn find_by_id(state: &AppState, id: Uuid) -> Result<Option<Self>, ApiError> {
        Ok(UserRepositoryImpl::find_by_id(state, id).await?)
    }

    pub async fn create(state: &AppState, payload: &CreateUserPayload) -> Result<Self, ApiError> {
        Ok(UserRepositoryImpl::create(state, payload).await?)
    }

    pub async fn update(state: &AppState, payload: &UpdateUserPayload) -> Result<Uuid, ApiError> {
        Ok(UserRepositoryImpl::update(state, payload).await?)
    }

    pub async fn delete(state: &AppState, payload: &DeletePayload) -> Result<(), ApiError> {
        Ok(UserRepositoryImpl::delete(state, payload).await?)
    }
}
