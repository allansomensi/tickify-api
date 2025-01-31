use crate::{errors::api_error::ApiError, models::ticket::TicketView};
use csv::WriterBuilder;

pub async fn create_ticket_csv(ticket: TicketView) -> Result<Vec<u8>, ApiError> {
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(vec![]);

    wtr.write_record(&[
        "Ticket",
        "Updated at",
        "Requester",
        "Created at",
        "Status",
        "Title",
        "Description",
        "Closed by",
        "Closed at",
        "Solution",
    ])?;

    wtr.write_record(&[
        ticket.id,
        ticket.updated_at,
        ticket.requester,
        ticket.created_at,
        ticket.status,
        ticket.title,
        ticket.description,
        ticket.closed_by,
        ticket.closed_at,
        ticket.solution,
    ])?;

    wtr.flush().unwrap();

    Ok(wtr.into_inner().unwrap())
}
