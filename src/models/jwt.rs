use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub iat: usize,
    pub sub: String,
    pub exp: usize,
}
