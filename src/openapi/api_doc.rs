use crate::{
    controllers::{auth, migrations, status, ticket, user},
    models::{status::Status, ticket::Ticket, user::User},
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
    paths(
        // Status
        status::show_status,

        // Migrations
        migrations::live_run,

        // Auth
        auth::login,
        auth::verify,

        // Users
        user::count_users,
        user::find_user_by_id,
        user::show_users,
        user::create_user,
        user::update_user,
        user::delete_user,

        // Tickets
        ticket::count_tickets,
        ticket::search_ticket,
        ticket::show_tickets,
        ticket::create_ticket,
        ticket::update_ticket,
        ticket::delete_ticket,
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
