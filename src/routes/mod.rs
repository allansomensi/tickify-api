pub mod auth;
pub mod migrations;
pub mod status;
pub mod swagger;
pub mod user;

use crate::{config::Config, database::AppState};
use axum::Router;
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest("/status", status::create_routes(state.clone()))
                .nest("/migrations", migrations::create_routes(state.clone()))
                .nest("/auth", auth::create_routes(state.clone()))
                .nest("/users", user::create_routes(state)),
        )
        .merge(swagger::swagger_routes())
        .layer(Config::cors())
}
