use crate::database::AppState;
use crate::models::ticket::TicketPublic;
use crate::models::user::{Role, Status, User};
use crate::models::{ticket::CreateTicketPayload, ticket::UpdateTicketPayload, DeletePayload};
use crate::validations::existence::ticket_exists;
use crate::{errors::api_error::ApiError, models::ticket::Ticket};
use axum::Extension;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use tracing::{debug, error, info};
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
    security(
        (),
        ("jwt_token" = ["jwt_token"])
    ),
    responses(
        (status = 200, description = "Ticket count retrieved successfully.", body = i32),
        (status = 500, description = "An error occurred while retrieving the ticket count.")
    )
)]
pub async fn count_tickets(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
) -> Result<impl IntoResponse, ApiError> {
    debug!("Received request to retrieve ticket count.");

    if current_user.status != Status::Active {
        return Err(ApiError::Unauthorized);
    }

    match Ticket::count(&state).await {
        Ok(count) => {
            info!("Successfully retrieved ticket count: {count}");
            Ok(Json(count))
        }
        Err(e) => {
            error!("Failed to retrieve ticket count: {e}");
            Err(ApiError::from(e))
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
    security(
        (),
        ("jwt_token" = ["jwt_token"])
    ),
    responses(
        (status = 200, description = "Tickets retrieved successfully.", body = Vec<TicketPublic>),
        (status = 404, description = "No tickets found in the database."),
        (status = 500, description = "An error occurred while retrieving the tickets.")
    )
)]
pub async fn find_all_tickets(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
) -> Result<impl IntoResponse, ApiError> {
    debug!("Received request to retrieve all tickets.");

    if current_user.status != Status::Active {
        return Err(ApiError::Unauthorized);
    }

    match Ticket::find_all(&state).await {
        Ok(tickets) => {
            info!("Tickets listed successfully.");
            Ok(Json(tickets))
        }
        Err(e) => {
            error!("Error retrieving all tickets: {e}");
            Err(ApiError::from(e))
        }
    }
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
    security(
        (),
        ("jwt_token" = ["jwt_token"])
    ),
    responses(
        (status = 200, description = "Ticket retrieved successfully.", body = Ticket),
        (status = 404, description = "No ticket found with the specified ID."),
        (status = 500, description = "An error occurred while retrieving the ticket.")
    )
)]
pub async fn find_ticket_by_id(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
) -> impl IntoResponse {
    debug!("Received request to retrieve ticket with id: {id}");

    if current_user.status != Status::Active {
        return Err(ApiError::Unauthorized);
    }

    if current_user.role != Role::Admin && current_user.role != Role::Moderator {
        return Err(ApiError::Unauthorized);
    }

    match Ticket::find_by_id(&state, id).await {
        Ok(Some(ticket)) => {
            info!("Ticket found: {id}");
            Ok(Json(ticket))
        }
        Ok(None) => {
            error!("No ticket found with id: {id}");
            Err(ApiError::NotFound)
        }
        Err(e) => {
            error!("Error retrieving ticket with id {id}: {e}");
            Err(ApiError::from(e))
        }
    }
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
    security(
        (),
        ("jwt_token" = ["jwt_token"])
    ),
    responses(
        (status = 201, description = "Ticket created successfully.", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long."),
        (status = 409, description = "Conflict: Ticket with the same name already exists."),
        (status = 500, description = "An error occurred while creating the ticket.")
    )
)]
pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<CreateTicketPayload>,
) -> Result<impl IntoResponse, ApiError> {
    debug!(
        "Received request to create ticket with title: {}",
        payload.title
    );

    if current_user.status != Status::Active {
        return Err(ApiError::Unauthorized);
    }

    // If not admin, get the requester from the JWT
    let payload = if current_user.role != Role::Admin && current_user.role != Role::Moderator {
        CreateTicketPayload {
            title: payload.title,
            description: payload.description,
            requester: Some(current_user.username),
        }
    } else {
        if payload.requester.is_some() {
            CreateTicketPayload {
                title: payload.title,
                description: payload.description,
                requester: payload.requester,
            }
        } else {
            CreateTicketPayload {
                title: payload.title,
                description: payload.description,
                requester: Some(current_user.username),
            }
        }
    };

    // Validations
    payload.validate()?;

    match Ticket::create(&state, &payload).await {
        Ok(new_ticket) => {
            info!("Ticket created! ID: {}", &new_ticket.id);
            Ok((StatusCode::CREATED, Json(new_ticket.id)))
        }
        Err(e) => {
            error!("Error creating ticket with title '{}': {e}", payload.title);
            Err(ApiError::from(e))
        }
    }
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
    security(
        (),
        ("jwt_token" = ["jwt_token"])
    ),
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
    Extension(current_user): Extension<User>,
    Json(payload): Json<UpdateTicketPayload>,
) -> Result<impl IntoResponse, ApiError> {
    debug!("Received request to update ticket with ID: {}", payload.id);

    if current_user.status != Status::Active {
        return Err(ApiError::Unauthorized);
    }

    // If not admin, ignore requester, status, closed_by and solution fields
    let payload = if current_user.role != Role::Admin && current_user.role != Role::Moderator {
        UpdateTicketPayload {
            id: payload.id,
            title: payload.title,
            description: payload.description,
            status: None,
            requester: None,
            closed_by: None,
            solution: None,
        }
    } else {
        UpdateTicketPayload {
            id: payload.id,
            title: payload.title,
            description: payload.description,
            status: payload.status,
            requester: payload.requester,
            closed_by: payload.closed_by,
            solution: payload.solution,
        }
    };

    // Validations
    payload.validate()?;
    ticket_exists(&state, payload.id).await?;

    match Ticket::update(&state, &payload).await {
        Ok(ticket_id) => {
            info!("Ticket updated! ID: {ticket_id}");
            Ok(Json(ticket_id))
        }
        Err(e) => {
            error!("Error updating ticket with ID {}: {e}", payload.id);
            Err(ApiError::from(e))
        }
    }
}

/// Deletes an existing ticket.
///
/// This endpoint allows tickets to delete a specific ticket by its ID.
/// It checks if the ticket exists before attempting to delete it.
/// If the ticket is successfully deleted, a 204 status code is returned.
#[utoipa::path(
    delete,
     path = "/api/v1/tickets",
     tags = ["Tickets"],
     summary = "Delete an existing ticket.",
     description = "This endpoint deletes a specific ticket from the database using its ID.",
     request_body = DeletePayload,
     security(
        (),
        ("jwt_token" = ["jwt_token"])
    ),
     responses(
         (status = 200, description = "Ticket deleted successfully"),
         (status = 404, description = "Ticket ID not found"),
         (status = 500, description = "An error occurred while deleting the ticket")
     )
 )]
pub async fn delete_ticket(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<DeletePayload>,
) -> Result<impl IntoResponse, ApiError> {
    debug!("Received request to delete ticket with ID: {}", payload.id);

    if current_user.status != Status::Active {
        return Err(ApiError::Unauthorized);
    }

    if current_user.role != Role::Admin && current_user.role != Role::Moderator {
        return Err(ApiError::Unauthorized);
    }

    // Validations
    ticket_exists(&state, payload.id).await?;

    match Ticket::delete(&state, &payload).await {
        Ok(_) => {
            info!("Ticket deleted! ID: {}", &payload.id);
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            error!("Error deleting ticket with ID {}: {e}", payload.id);
            Err(ApiError::from(e))
        }
    }
}
