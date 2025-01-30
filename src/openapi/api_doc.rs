use crate::{
    controllers::{auth, export, migrations, status, ticket, user},
    models::{status::Status, ticket::Ticket, user::User},
};
use serde::Serialize;
use utoipa::{
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
    Modify,
};

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Tickify API",
        description = "A simple REST API using Axum for support ticket management.",
        contact(name = "Allan Somensi", email = "allansomensidev@gmail.com"),
        license(name = "MIT", identifier = "MIT")
    ),
    servers(
        (url = "http://localhost:8000", description = "Local server"),
    ),
    modifiers(&AuthToken),
    paths(
        // Status
        status::show_status,

        // Migrations
        migrations::live_run,

        // Auth
        auth::login,
        auth::register,
        auth::verify,

        // Users
        user::count_users,
        user::find_user_by_id,
        user::find_all_users,
        user::create_user,
        user::update_user,
        user::delete_user,

        // Tickets
        ticket::count_tickets,
        ticket::find_ticket_by_id,
        ticket::find_all_tickets,
        ticket::create_ticket,
        ticket::update_ticket,
        ticket::delete_ticket,

        // Export
        export::ticket_to_pdf,
    ),
    components(
        schemas(Status, User, Ticket)
    ),
    tags(
        (name = "Status", description = "Status endpoints"),
        (name = "Migrations", description = "Migrations endpoints"),
        (name = "Auth", description = "Auth endpoints"),
        (name = "Users", description = "Users endpoints"),
        (name = "Tickets", description = "Tickets endpoints"),
    )
)]
pub struct ApiDoc;

#[derive(Debug, Serialize)]
struct AuthToken;

impl Modify for AuthToken {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "jwt_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
