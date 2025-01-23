use crate::{database::AppState, errors::api_error::ApiError, models::ticket::Ticket};
use tracing::debug;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TicketRepository {
    async fn count(state: &AppState) -> Result<i64, ApiError>;
    async fn find_all(state: &AppState) -> Result<Vec<Ticket>, ApiError>;
    async fn find_by_id(state: &AppState, id: Uuid) -> Result<Option<Ticket>, ApiError>;
    // TODO!
    // async fn create(state: &AppState, payload: &CreateTicketPayload) -> Result<Ticket, ApiError>;
    // async fn update(state: &AppState, payload: &UpdateTicketPayload) -> Result<Uuid, ApiError>;
    // async fn delete(state: &AppState, payload: &DeletePayload) -> Result<(), ApiError>;
}

pub struct TicketRepositoryImpl;

#[async_trait::async_trait]
impl TicketRepository for TicketRepositoryImpl {
    async fn count(state: &AppState) -> Result<i64, ApiError> {
        debug!("Attempting to count tickets from the database...");

        let count: i64 = sqlx::query_scalar(r#"SELECT COUNT(*) FROM tickets;"#)
            .fetch_one(&state.db)
            .await?;

        Ok(count)
    }

    async fn find_all(state: &AppState) -> Result<Vec<Ticket>, ApiError> {
        debug!("Attempting to retrieve all tickets from the database...");

        let tickets: Vec<Ticket> = sqlx::query_as(r#"SELECT * FROM tickets;"#)
            .fetch_all(&state.db)
            .await?;

        Ok(tickets)
    }

    async fn find_by_id(state: &AppState, id: Uuid) -> Result<Option<Ticket>, ApiError> {
        debug!("Attempting to retrieve ticket with id: {id}");

        let ticket: Option<Ticket> = sqlx::query_as(r#"SELECT * FROM tickets WHERE id = $1;"#)
            .bind(id)
            .fetch_optional(&state.db)
            .await?;

        Ok(ticket)
    }
}
