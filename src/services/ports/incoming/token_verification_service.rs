use std::{error::Error, fmt::Display};

use async_trait::async_trait;

use crate::services::domain::user::User;

#[async_trait]
pub trait TokenVerificationService {
    async fn verify_token(&self, token: String) -> Result<User, TokenVerificationServiceError>;
}

#[derive(Debug, PartialEq)]
pub enum TokenVerificationServiceError {
    InternalError,
    TokenDecoding,
    UserNotFound,
}

impl Error for TokenVerificationServiceError {}

impl Display for TokenVerificationServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenVerificationServiceError::UserNotFound => f.write_str("User not found"),
            TokenVerificationServiceError::InternalError => f.write_str("Internal error"),
            TokenVerificationServiceError::TokenDecoding => f.write_str("Token decoding error"),
        }
    }
}
