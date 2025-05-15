use tickify_api::{config, errors, server};

#[tokio::main]
async fn main() -> Result<(), errors::api_error::ApiError> {
    println!("🌟 Tickify API 🌟");

    match config::Config::init() {
        Ok(_) => {
            tracing::info!("✅ Configurations loaded!");
        }
        Err(e) => {
            tracing::error!("❌ Error loading configurations: {e}");
            std::process::exit(1);
        }
    }

    Ok(server::run().await?)
}
