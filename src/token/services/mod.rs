mod jwt_service;
mod token_service;

pub use jwt_service::JwtService;
use serde::{Deserialize, Serialize};
pub use token_service::TokenService;

#[derive(Serialize, Deserialize)]
pub struct TokenBundle {
    pub access_token: String,
    pub refresh_token: String,
}
