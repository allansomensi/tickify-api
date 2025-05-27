use crate::{
    database::AppState,
    errors::api_error::ApiError,
    export::{csv::create_tickets_csv, pdf::create_ticket_pdf},
    models::{
        ticket::{Ticket, TicketView},
        user::{Status, User},
    },
};
use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{header, HeaderMap},
    response::IntoResponse,
    Extension,
};
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/api/v1/export/pdf/ticket/{id}",
    tags = ["Tickets"],
    summary = "Generates a ticket in PDF.",
    description = "Generates a PDF with the ticket information by its ID.",
    params(
        ("id", description = "The unique identifier of the ticket.", example = Uuid::new_v4)
    ),
    security(
        (),
        ("jwt_token" = ["jwt_token"])
    )
)]
pub async fn ticket_to_pdf(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
) -> Result<impl IntoResponse, ApiError> {
    debug!("Received a request to generate a PDF of the ticket with ID: {id}");

    if current_user.status != Status::Active {
        return Err(ApiError::Unauthorized);
    }

    let ticket = match Ticket::find_by_id(&state, id).await {
        Ok(Some(ticket)) => Ok(ticket),
        Ok(None) => {
            error!("No ticket found with id: {id}");
            Err(ApiError::NotFound)
        }
        Err(e) => {
            error!("Error finding the ticket with ID {id} while generating its PDF: {e}");
            Err(ApiError::from(e))
        }
    }?;

    // Formats all fields for PDF generation.
    // For each field not found, returns `null`.

    let id = ticket.id.to_string();
    let formatted_status = ticket.status.to_string();
    let formatted_solution = if let Some(solution) = ticket.solution {
        solution
    } else {
        "null".to_string()
    };

    let time_fmt = "%Y-%m-%d %H:%M:%S";

    let formatted_created_at = ticket.created_at.format(time_fmt).to_string();
    let formatted_updated_at = ticket.updated_at.format(time_fmt).to_string();

    let formatted_closed_at = if let Some(closed_at) = ticket.closed_at {
        closed_at.format(time_fmt).to_string()
    } else {
        "null".to_string()
    };

    let formatted_closed_by = if let Some(closed_by) = ticket.closed_by {
        closed_by.username.to_string()
    } else {
        "null".to_string()
    };

    let formatted_ticket = TicketView {
        id,
        title: ticket.title,
        description: ticket.description,
        requester: ticket.requester.username,
        status: formatted_status,
        closed_by: formatted_closed_by,
        solution: formatted_solution,
        created_at: formatted_created_at,
        updated_at: formatted_updated_at,
        closed_at: formatted_closed_at,
    };

    let pdf = create_ticket_pdf(formatted_ticket).await;

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        "application/pdf; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"Ticket.pdf\"".parse().unwrap(),
    );

    let body = Bytes::from(pdf?);

    Ok((headers, body))
}

#[utoipa::path(
    get,
    path = "/api/v1/export/csv/ticket/{id}",
    tags = ["Tickets"],
    summary = "Generates a ticket in CSV.",
    description = "Generates a CSV with the ticket information by its ID.",
    params(
        ("id", description = "The unique identifier of the ticket.", example = Uuid::new_v4)
    ),
    security(
        (),
        ("jwt_token" = ["jwt_token"])
    )
)]
pub async fn ticket_to_csv(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
) -> Result<impl IntoResponse, ApiError> {
    debug!("Received a request to generate a PDF of the ticket with ID: {id}");

    if current_user.status != Status::Active {
        return Err(ApiError::Unauthorized);
    }

    let ticket = match Ticket::find_by_id(&state, id).await {
        Ok(Some(ticket)) => Ok(ticket),
        Ok(None) => {
            error!("No ticket found with id: {id}");
            Err(ApiError::NotFound)
        }
        Err(e) => {
            error!("Error finding the ticket with ID {id} while generating its PDF: {e}");
            Err(ApiError::from(e))
        }
    }?;

    // Formats all fields for PDF generation.
    // For each field not found, returns `null`.

    let id = ticket.id.to_string();

    let formatted_status = ticket.status.to_string();

    let formatted_solution = if let Some(solution) = ticket.solution {
        solution
    } else {
        "null".to_string()
    };

    let time_fmt = "%Y-%m-%d %H:%M:%S";

    let formatted_created_at = ticket.created_at.format(time_fmt).to_string();
    let formatted_updated_at = ticket.updated_at.format(time_fmt).to_string();

    let formatted_closed_at = if let Some(closed_at) = ticket.closed_at {
        closed_at.format(time_fmt).to_string()
    } else {
        "null".to_string()
    };

    let formatted_closed_by = if let Some(closed_by) = ticket.closed_by {
        closed_by.username.to_string()
    } else {
        "null".to_string()
    };

    let formatted_ticket = TicketView {
        id,
        title: ticket.title,
        description: ticket.description,
        requester: ticket.requester.username,
        status: formatted_status,
        closed_by: formatted_closed_by,
        solution: formatted_solution,
        created_at: formatted_created_at,
        updated_at: formatted_updated_at,
        closed_at: formatted_closed_at,
    };

    let mut tickets = Vec::new();
    tickets.push(formatted_ticket);

    let csv = create_tickets_csv(tickets).await.unwrap();

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        "application/csv; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"Ticket.csv\"".parse().unwrap(),
    );

    let body = Bytes::from(csv);

    Ok((headers, body))
}

#[utoipa::path(
    get,
    path = "/api/v1/export/csv/tickets",
    tags = ["Tickets"],
    summary = "Generates a CSV of all tickets.",
    description = "Generates a CSV with all tickets in the system.",
    security(
        (),
        ("jwt_token" = ["jwt_token"])
    )
)]
pub async fn tickets_to_csv(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<User>,
) -> Result<impl IntoResponse, ApiError> {
    debug!("Received a request to generate a CSV of all tickets");

    if current_user.status != Status::Active {
        return Err(ApiError::Unauthorized);
    }

    let tickets = match Ticket::find_all(&state).await {
        Ok(tickets) => tickets,
        Err(e) => {
            error!("Error fetching all tickets for CSV export: {e}");
            return Err(ApiError::from(e));
        }
    };

    // Formatar todos os tickets
    let mut ticket_views = Vec::new();

    for ticket in tickets {
        let id = ticket.id.to_string();

        let formatted_status = ticket.status.to_string();
        let formatted_solution = ticket.solution.unwrap_or_else(|| "null".to_string());

        let time_fmt = "%Y-%m-%d %H:%M:%S";

        let formatted_created_at = ticket.created_at.format(time_fmt).to_string();
        let formatted_updated_at = ticket.updated_at.format(time_fmt).to_string();
        let formatted_closed_at = ticket.closed_at.map_or("null".to_string(), |closed_at| {
            closed_at.format(time_fmt).to_string()
        });

        let closed_by_username = if let Some(closed_by) = ticket.closed_by {
            closed_by.username.to_string()
        } else {
            "null".to_string()
        };

        ticket_views.push(TicketView {
            id,
            title: ticket.title,
            description: ticket.description,
            requester: ticket.requester.username,
            status: formatted_status,
            closed_by: closed_by_username,
            solution: formatted_solution,
            created_at: formatted_created_at,
            updated_at: formatted_updated_at,
            closed_at: formatted_closed_at,
        });
    }

    // Gerar o CSV a partir de todos os tickets
    let csv = create_tickets_csv(ticket_views).await.unwrap();

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        "application/csv; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"Tickets.csv\"".parse().unwrap(),
    );

    let body = Bytes::from(csv);

    Ok((headers, body))
}
