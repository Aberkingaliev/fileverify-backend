mod jwt_service;
mod token_service;

pub use jwt_service::JwtService;
pub use token_service::TokenService;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenBundle {
    pub access_token: String,
    pub refresh_token: String,
}
