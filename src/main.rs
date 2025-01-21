mod config;
mod controllers;
mod database;
mod errors;
mod models;
mod openapi;
mod routes;
mod server;
mod utils;
mod validations;

#[tokio::main]
async fn main() {
    println!("🌟 Tickify API 🌟");

    match config::Config::init() {
        Ok(_) => {
            tracing::info!("✅ Configurations loaded!");
        }
        Err(e) => {
            tracing::error!("❌ Error loading configurations: {:?}", e);
            std::process::exit(1);
        }
    }
    server::run().await.unwrap();
}
