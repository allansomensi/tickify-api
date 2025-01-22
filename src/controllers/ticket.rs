use crate::database::AppState;
use crate::models::ticket::{TicketStatus, UpdateTicketPayload};
use crate::models::{ticket::CreateTicketPayload, DeletePayload};
use crate::validations::existence::ticket_exists;
use crate::{errors::api_error::ApiError, models::ticket::Ticket};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

/// Retrieves the total count of tickets.
///
/// This endpoint counts all tickets stored in the database and returns the count as an integer.
/// If no tickets are found, 0 is returned.
#[utoipa::path(
    get,
    path = "/api/v1/tickets/count",
    tags = ["Tickets"],
    summary = "Get the total count of tickets.",
    description = "This endpoint retrieves the total number of tickets stored in the database.",
    responses(
        (status = 200, description = "Ticket count retrieved successfully.", body = i32),
        (status = 500, description = "An error occurred while retrieving the ticket count.")
    )
)]
pub async fn count_tickets(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let count = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM tickets;"#)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving ticket count: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Successfully retrieved ticket count: {count}");
    Ok(Json(count))
}

/// Retrieves a specific ticket by its ID.
///
/// This endpoint searches for a ticket with the specified ID.
/// If the ticket is found, it returns the ticket details.
#[utoipa::path(
    get,
    path = "/api/v1/tickets/{id}",
    tags = ["Tickets"],
    summary = "Get a specific ticket by ID.",
    description = "This endpoint retrieves a ticket's details from the database using its ID. Returns the ticket if found, or a 404 status if not found.",
    params(
        ("id", description = "The unique identifier of the ticket to retrieve.", example = Uuid::new_v4)
    ),
    responses(
        (status = 200, description = "Ticket retrieved successfully.", body = Ticket),
        (status = 404, description = "No ticket found with the specified ID."),
        (status = 500, description = "An error occurred while retrieving the ticket.")
    )
)]
pub async fn search_ticket(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let ticket = sqlx::query_as::<_, Ticket>(r#"SELECT * FROM tickets WHERE id = $1;"#)
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving ticket with id {id}: {e}");
            ApiError::DatabaseError(e)
        })?;

    match ticket {
        Some(ticket) => {
            info!("Ticket found: {id}");
            Ok(Json(ticket))
        }
        None => {
            error!("No ticket found with id: {id}");
            Err(ApiError::NotFound)
        }
    }
}

/// Retrieves a list of all tickets.
///
/// This endpoint fetches all tickets stored in the database.
/// If there are no tickets, returns an empty array.
#[utoipa::path(
    get,
    path = "/api/v1/tickets",
    tags = ["Tickets"],
    summary = "List all tickets.",
    description = "Fetches all tickets stored in the database. If there are no tickets, returns an empty array.",
    responses(
        (status = 200, description = "Tickets retrieved successfully.", body = Vec<Ticket>),
        (status = 404, description = "No tickets found in the database."),
        (status = 500, description = "An error occurred while retrieving the tickets.")
    )
)]
pub async fn show_tickets(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let tickets = sqlx::query_as::<_, Ticket>(r#"SELECT * FROM tickets;"#)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            error!("Error listing tickets: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Tickets listed successfully");
    Ok(Json(tickets))
}

/// Create a new ticket.
///
/// This endpoint creates a new ticket by providing its details.
/// Validates the ticket's name for length and emptiness, checks for duplicates,
/// and inserts the new ticket into the database if all validations pass.
#[utoipa::path(
    post,
    path = "/api/v1/tickets",
    tags = ["Tickets"],
    summary = "Create a new ticket.",
    description = "This endpoint creates a new ticket in the database with the provided details.",
    request_body = CreateTicketPayload,
    responses(
        (status = 201, description = "Ticket created successfully.", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long."),
        (status = 409, description = "Conflict: Ticket with the same name already exists."),
        (status = 500, description = "An error occurred while creating the ticket.")
    )
)]
pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTicketPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    payload.validate()?;

    let requester_id: Uuid =
        sqlx::query_scalar(r#"SELECT id FROM users WHERE username = $1 LIMIT 1"#)
            .bind(&payload.requester)
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                error!("Error fetching requester ID: {e}");
                ApiError::DatabaseError(e)
            })?;

    let new_ticket = Ticket::new(&payload.title, &payload.description, requester_id);

    // Creates the ticket.
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
        .await
        .map_err(|e| {
            error!("Error creating ticket: {e}");
            ApiError::DatabaseError(e)
        })?;
    info!("Ticket created! ID: {}", &new_ticket.id);
    Ok((StatusCode::CREATED, Json(new_ticket.id)))
}

/// Updates an existing ticket.
///
/// This endpoint updates the details of an existing ticket.
/// It accepts the ticket ID and the new details for the ticket.
/// The endpoint validates the new name to ensure it is not empty,
/// does not conflict with an existing ticket's name, and meets length requirements.
/// If the ticket is successfully updated, it returns the UUID of the updated ticket.
#[utoipa::path(
    put,
    path = "/api/v1/tickets",
    tags = ["Tickets"],
    summary = "Update an existing ticket.",
    description = "This endpoint updates the details of an existing ticket in the database.",
    request_body = UpdateTicketPayload,
    responses(
        (status = 200, description = "Ticket updated successfully.", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long."),
        (status = 404, description = "Ticket ID not found."),
        (status = 409, description = "Conflict: Ticket with the same name already exists."),
        (status = 500, description = "An error occurred while updating the ticket.")
    )
)]
pub async fn update_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateTicketPayload>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    payload.validate()?;
    ticket_exists(state.clone(), payload.id).await?;

    let ticket_id = payload.id;
    let new_title = payload.title;
    let new_description = payload.description;
    let new_requester = payload.requester;
    let new_status = payload.status;
    let new_closed_by = payload.closed_by;
    let new_solution = payload.solution;

    let mut updated = false;

    // Update `title` if provided.
    if let Some(title) = new_title {
        sqlx::query(r#"UPDATE tickets SET title = $1 WHERE id = $2;"#)
            .bind(title)
            .bind(ticket_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating title: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update `description` if provided.
    if let Some(description) = new_description {
        sqlx::query(r#"UPDATE tickets SET description = $1 WHERE id = $2;"#)
            .bind(description)
            .bind(ticket_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating description: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update `requester` if provided
    if let Some(requester) = new_requester {
        sqlx::query(r#"UPDATE tickets SET requester = $1 WHERE id = $2;"#)
            .bind(requester)
            .bind(ticket_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating requester: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update `status` if provided
    if let Some(status) = new_status {
        // Checks previous status value
        let previous_status: Option<TicketStatus> =
            sqlx::query_scalar(r#"SELECT status FROM tickets WHERE id = $1"#)
                .bind(ticket_id)
                .fetch_optional(&state.db)
                .await
                .map_err(|e| {
                    error!("Error fetching previous status: {e}");
                    ApiError::DatabaseError(e)
                })?;

        // Update to new value
        sqlx::query(r#"UPDATE tickets SET status = $1 WHERE id = $2;"#)
            .bind(status.clone())
            .bind(ticket_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating status: {e}");
                ApiError::DatabaseError(e)
            })?;

        // Checks if the status has changed to `Closed` or `Cancelled`
        if status == TicketStatus::Closed || status == TicketStatus::Cancelled {
            if let Some(prev_status) = previous_status {
                // If the previous status was not "Closed" or "Cancelled", update the `closed_at` field
                if prev_status != TicketStatus::Closed || prev_status != TicketStatus::Cancelled {
                    sqlx::query(r#"UPDATE tickets SET closed_at = $1 WHERE id = $2;"#)
                        .bind(Utc::now().naive_utc())
                        .bind(ticket_id)
                        .execute(&state.db)
                        .await
                        .map_err(|e| {
                            error!("Error updating closed_at: {e}");
                            ApiError::DatabaseError(e)
                        })?;
                }
            }
        }
        updated = true;
    }

    // Update `closed_by` if provided.
    if let Some(closed_by) = new_closed_by {
        sqlx::query(r#"UPDATE tickets SET closed_by = $1 WHERE id = $2;"#)
            .bind(closed_by)
            .bind(ticket_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating closed_by: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update `solution` if provided
    if let Some(solution) = new_solution {
        sqlx::query(r#"UPDATE tickets SET solution = $1 WHERE id = $2;"#)
            .bind(solution)
            .bind(ticket_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating solution: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update `updated_at` field.
    if updated {
        sqlx::query(r#"UPDATE tickets SET updated_at = $1 WHERE id = $2;"#)
            .bind(Utc::now().naive_utc())
            .bind(ticket_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating last_name: {e}");
                ApiError::DatabaseError(e)
            })?;
    } else {
        error!(
            "No updates were made for the provided ticket ID: {}",
            &ticket_id
        );
        return Err(ApiError::NotModified);
    }

    info!("Ticket updated! ID: {}", &ticket_id);
    Ok((StatusCode::OK, Json(ticket_id)).into_response())
}

/// Deletes an existing ticket.
///
/// This endpoint allows tickets to delete a specific ticket by its ID.
/// It checks if the ticket exists before attempting to delete it.
/// If the ticket is successfully deleted, a confirmation message is returned.
#[utoipa::path(
    delete,
     path = "/api/v1/tickets",
     tags = ["Tickets"],
     summary = "Delete an existing ticket.",
     description = "This endpoint deletes a specific ticket from the database using its ID.",
     request_body = DeletePayload,
     responses(
         (status = 200, description = "Ticket deleted successfully"),
         (status = 404, description = "Ticket ID not found"),
         (status = 500, description = "An error occurred while deleting the ticket")
     )
 )]
pub async fn delete_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeletePayload>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    ticket_exists(state.clone(), payload.id).await?;

    // Delete the ticket
    sqlx::query(r#"DELETE FROM tickets WHERE id = $1;"#)
        .bind(payload.id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error deleting ticket: {}", e);
            ApiError::DatabaseError(e)
        })?;

    info!("Ticket deleted! ID: {}", &payload.id);
    Ok((StatusCode::OK, Json("Ticket deleted!")))
}
