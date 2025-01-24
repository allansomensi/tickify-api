pub mod auth;
pub mod migrations;
pub mod status;
pub mod swagger;
pub mod ticket;
pub mod user;

use crate::{config::Config, database::AppState, middlewares::authorization::authorize};
use axum::{middleware, Router};
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest("/users", user::create_routes(state.clone()))
                .nest("/tickets", ticket::create_routes(state.clone()))
                .layer(middleware::from_fn_with_state(state.clone(), authorize))
                .nest("/auth", auth::create_routes(state.clone()))
                .nest("/status", status::create_routes(state.clone()))
                .nest("/migrations", migrations::create_routes(state.clone())),
        )
        .merge(swagger::swagger_routes())
        .layer(Config::cors())
}
