use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct LoginPayload {
    #[validate(length(min = 3, message = "Password must be between 3 and 20 characters long"))]
    pub username: String,
    #[validate(length(
        min = 8,
        max = 100,
        message = "Password must be between 8 and 100 chars."
    ))]
    pub password: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct VerifyTokenPayload {
    pub token: String,
}
