use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing token")]
    MissingToken,
    #[error("Empty header")]
    EmptyHeader,
    #[error("Invalid Token")]
    InvalidToken,
}
