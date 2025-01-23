use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::{
        repositories::ticket_repository::{TicketRepository, TicketRepositoryImpl},
        AppState,
    },
    errors::api_error::ApiError,
};

use super::DeletePayload;

#[derive(ToSchema, PartialEq, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "ticket_status", rename_all = "snake_case")]
pub enum TicketStatus {
    Open,
    InProgress,
    Closed,
    Reopened,
    Paused,
    Cancelled,
}

#[derive(ToSchema, FromRow, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub requester: Uuid,
    pub status: TicketStatus,
    pub closed_by: Option<Uuid>,
    pub solution: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateTicketPayload {
    #[validate(length(min = 3, max = 50, message = "Title must be between 3 and 50 chars."))]
    pub title: String,
    #[validate(length(
        min = 10,
        max = 3000,
        message = "Description must be between 3 and 3000 chars."
    ))]
    pub description: String,
    pub requester: String,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateTicketPayload {
    pub id: Uuid,
    #[validate(length(min = 3, max = 50, message = "Title must be between 3 and 50 chars."))]
    pub title: Option<String>,
    #[validate(length(
        min = 10,
        max = 3000,
        message = "Description must be between 10 and 3000 chars."
    ))]
    pub description: Option<String>,
    pub requester: Option<Uuid>,
    pub status: Option<TicketStatus>,
    pub closed_by: Option<Uuid>,
    #[validate(length(
        min = 10,
        max = 3000,
        message = "Description must be between 10 and 3000 chars."
    ))]
    pub solution: Option<String>,
}

impl Ticket {
    pub fn new(title: &str, description: &str, requester: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: description.to_string(),
            requester,
            status: TicketStatus::Open,
            closed_by: None,
            solution: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            closed_at: None,
        }
    }

    pub async fn count(state: &AppState) -> Result<i64, ApiError> {
        Ok(TicketRepositoryImpl::count(state).await?)
    }

    pub async fn find_all(state: &AppState) -> Result<Vec<Self>, ApiError> {
        Ok(TicketRepositoryImpl::find_all(state).await?)
    }

    pub async fn find_by_id(state: &AppState, id: Uuid) -> Result<Option<Self>, ApiError> {
        Ok(TicketRepositoryImpl::find_by_id(state, id).await?)
    }

    pub async fn create(state: &AppState, payload: &CreateTicketPayload) -> Result<Self, ApiError> {
        Ok(TicketRepositoryImpl::create(state, payload).await?)
    }

    pub async fn update(state: &AppState, payload: &UpdateTicketPayload) -> Result<Uuid, ApiError> {
        Ok(TicketRepositoryImpl::update(state, payload).await?)
    }

    pub async fn delete(state: &AppState, payload: &DeletePayload) -> Result<(), ApiError> {
        Ok(TicketRepositoryImpl::delete(state, payload).await?)
    }
}
