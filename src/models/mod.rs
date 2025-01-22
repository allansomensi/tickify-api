use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

pub mod auth;
pub mod jwt;
pub mod status;
pub mod ticket;
pub mod user;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct DeletePayload {
    pub id: Uuid,
}
