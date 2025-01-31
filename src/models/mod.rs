pub mod auth;
pub mod status;
pub mod ticket;
pub mod user;

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct DeletePayload {
    pub id: uuid::Uuid,
}
