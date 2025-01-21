use crate::database::AppState;
use crate::errors::api_error::ApiError;
use std::sync::Arc;
use tracing::error;

/// Check if there is already another user with the same username.
pub async fn is_user_unique(state: Arc<AppState>, username: String) -> Result<(), ApiError> {
    let exists = sqlx::query(r#"SELECT id FROM users WHERE username = $1;"#)
        .bind(&username)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error checking for existing username: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if exists {
        error!("Username '{}' already exists.", &username);
        Err(ApiError::AlreadyExists)
    } else {
        Ok(())
    }
}
