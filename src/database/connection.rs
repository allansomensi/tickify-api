use sqlx::PgPool;
use std::env;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("Failed to load DATABASE_URL");

    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}
