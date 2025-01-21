use crate::{
    controllers::{auth, migrations, status, user},
    models::{status::Status, user::User},
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
        user::search_user,
        user::show_users,
        user::create_user,
        user::update_user,
        user::delete_user,
    ),
    components(
        schemas(Status, User)
    ),
    tags(
        (name = "Status", description = "Status endpoints"),
        (name = "Migrations", description = "Migrations endpoints"),
        (name = "Auth", description = "Auth endpoints"),
        (name = "Users", description = "Users endpoints"),
    )
)]
pub struct ApiDoc;
