use crate::{controllers::export, database::AppState};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/pdf/ticket/{id}", get(export::ticket_to_pdf))
        .route("/csv/tickets", get(export::tickets_to_csv))
        .route("/csv/ticket/{id}", get(export::ticket_to_csv))
        .with_state(state)
}
