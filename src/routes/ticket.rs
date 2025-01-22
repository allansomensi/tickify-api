use crate::{controllers::ticket, database::AppState};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/count", get(ticket::count_tickets))
        .route("/{id}", get(ticket::search_ticket))
        .route(
            "/",
            get(ticket::show_tickets)
                .post(ticket::create_ticket)
                .put(ticket::update_ticket)
                .delete(ticket::delete_ticket),
        )
        .with_state(state)
}
