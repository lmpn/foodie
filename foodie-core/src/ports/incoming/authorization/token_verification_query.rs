use crate::domain::authorization::user::User;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait TokenVerificationQuery {
    async fn verify_token(&self, token: String) -> Result<User, TokenVerificationQueryError>;
}

#[derive(Debug, PartialEq)]
pub enum TokenVerificationQueryError {
    InternalError,
    TokenDecoding,
    UserNotFound,
}

impl Error for TokenVerificationQueryError {}

impl Display for TokenVerificationQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenVerificationQueryError::UserNotFound => f.write_str("User not found"),
            TokenVerificationQueryError::InternalError => f.write_str("Internal error"),
            TokenVerificationQueryError::TokenDecoding => f.write_str("Token decoding error"),
        }
    }
}
