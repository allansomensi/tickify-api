use crate::{
    database::AppState,
    errors::api_error::ApiError,
    models::{
        ticket::{CreateTicketPayload, Ticket, TicketStatus, UpdateTicketPayload},
        DeletePayload,
    },
};
use tracing::{debug, info};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TicketRepository {
    async fn count(state: &AppState) -> Result<i64, ApiError>;
    async fn find_all(state: &AppState) -> Result<Vec<Ticket>, ApiError>;
    async fn find_by_id(state: &AppState, id: Uuid) -> Result<Option<Ticket>, ApiError>;
    async fn create(state: &AppState, payload: &CreateTicketPayload) -> Result<Ticket, ApiError>;
    async fn update(state: &AppState, payload: &UpdateTicketPayload) -> Result<Uuid, ApiError>;
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

    async fn update(state: &AppState, payload: &UpdateTicketPayload) -> Result<Uuid, ApiError> {
        debug!("Attempting to update ticket with ID: {}", payload.id);

        let ticket_id = payload.id;
        let new_title = &payload.title;
        let new_description = &payload.description;
        let new_requester = payload.requester;
        let new_status = &payload.status;
        let new_closed_by = payload.closed_by;
        let new_solution = &payload.solution;

        let mut updated = false;

        // Update `title` if provided.
        if let Some(title) = new_title {
            sqlx::query(r#"UPDATE tickets SET title = $1 WHERE id = $2;"#)
                .bind(title)
                .bind(ticket_id)
                .execute(&state.db)
                .await?;

            info!("Updated title of ticket with ID: {}", payload.id);
            updated = true;
        }

        // Update `description` if provided.
        if let Some(description) = new_description {
            sqlx::query(r#"UPDATE tickets SET description = $1 WHERE id = $2;"#)
                .bind(description)
                .bind(ticket_id)
                .execute(&state.db)
                .await?;

            info!("Updated description of ticket with ID: {}", payload.id);
            updated = true;
        }

        // Update `requester` if provided
        if let Some(requester) = new_requester {
            sqlx::query(r#"UPDATE tickets SET requester = $1 WHERE id = $2;"#)
                .bind(requester)
                .bind(ticket_id)
                .execute(&state.db)
                .await?;

            info!("Updated requester of ticket with ID: {}", payload.id);
            updated = true;
        }

        // Update `status` if provided
        if let Some(status) = new_status {
            // Checks previous status value
            let previous_status: Option<TicketStatus> =
                sqlx::query_scalar(r#"SELECT status FROM tickets WHERE id = $1"#)
                    .bind(ticket_id)
                    .fetch_optional(&state.db)
                    .await?;

            // Update to new value
            sqlx::query(r#"UPDATE tickets SET status = $1 WHERE id = $2;"#)
                .bind(status.clone())
                .bind(ticket_id)
                .execute(&state.db)
                .await?;

            // Checks if the status has changed to `Closed` or `Cancelled`
            if status == &TicketStatus::Closed || status == &TicketStatus::Cancelled {
                if let Some(prev_status) = previous_status {
                    // If the previous status was not "Closed" or "Cancelled", update the `closed_at` field
                    if prev_status != TicketStatus::Closed || prev_status != TicketStatus::Cancelled
                    {
                        sqlx::query(r#"UPDATE tickets SET closed_at = $1 WHERE id = $2;"#)
                            .bind(chrono::Utc::now().naive_utc())
                            .bind(ticket_id)
                            .execute(&state.db)
                            .await?;
                    }
                }
            }

            info!("Updated status of ticket with ID: {}", payload.id);
            updated = true;
        }

        // Update `closed_by` if provided.
        if let Some(closed_by) = new_closed_by {
            sqlx::query(r#"UPDATE tickets SET closed_by = $1 WHERE id = $2;"#)
                .bind(closed_by)
                .bind(ticket_id)
                .execute(&state.db)
                .await?;

            info!(
                "Updated `closed_by` field of ticket with ID: {}",
                payload.id
            );
            updated = true;
        }

        // Update `solution` if provided
        if let Some(solution) = new_solution {
            sqlx::query(r#"UPDATE tickets SET solution = $1 WHERE id = $2;"#)
                .bind(solution)
                .bind(ticket_id)
                .execute(&state.db)
                .await?;

            info!("Updated solution of ticket with ID: {}", payload.id);
            updated = true;
        }

        // Update `updated_at` field.
        if updated {
            sqlx::query(r#"UPDATE tickets SET updated_at = $1 WHERE id = $2;"#)
                .bind(chrono::Utc::now().naive_utc())
                .bind(ticket_id)
                .execute(&state.db)
                .await?;
        } else {
            return Err(ApiError::NotModified);
        }

        Ok(ticket_id)
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
