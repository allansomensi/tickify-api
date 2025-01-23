use crate::{
    database::AppState,
    errors::api_error::ApiError,
    models::{
        ticket::{CreateTicketPayload, Ticket},
        DeletePayload,
    },
};
use tracing::debug;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TicketRepository {
    async fn count(state: &AppState) -> Result<i64, ApiError>;
    async fn find_all(state: &AppState) -> Result<Vec<Ticket>, ApiError>;
    async fn find_by_id(state: &AppState, id: Uuid) -> Result<Option<Ticket>, ApiError>;
    async fn create(state: &AppState, payload: &CreateTicketPayload) -> Result<Ticket, ApiError>;
    // TODO!
    // async fn update(state: &AppState, payload: &UpdateTicketPayload) -> Result<Uuid, ApiError>;
    async fn delete(state: &AppState, payload: &DeletePayload) -> Result<(), ApiError>;
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

    async fn create(state: &AppState, payload: &CreateTicketPayload) -> Result<Ticket, ApiError> {
        debug!("Attempting to create ticket with title: {}", payload.title);

        let requester_id: Uuid =
            sqlx::query_scalar(r#"SELECT id FROM users WHERE username = $1 LIMIT 1"#)
                .bind(&payload.requester)
                .fetch_one(&state.db)
                .await?;

        let new_ticket = Ticket::new(&payload.title, &payload.description, requester_id);

        sqlx::query(r#"INSERT INTO tickets (id, title, description, requester, status, closed_by, solution, created_at, updated_at, closed_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#)
        .bind(new_ticket.id)
        .bind(&new_ticket.title)
        .bind(&new_ticket.description)
        .bind(new_ticket.requester)
        .bind(&new_ticket.status)
        .bind(new_ticket.closed_by)
        .bind(&new_ticket.solution)
        .bind(new_ticket.created_at)
        .bind(new_ticket.updated_at)
        .bind(new_ticket.closed_at)
        .execute(&state.db)
        .await?;

        Ok(new_ticket)
    }

    async fn delete(state: &AppState, payload: &DeletePayload) -> Result<(), ApiError> {
        debug!("Attempting to delete ticket with ID: {}", payload.id);

        sqlx::query(r#"DELETE FROM tickets WHERE id = $1;"#)
            .bind(payload.id)
            .execute(&state.db)
            .await?;

        Ok(())
    }
}
