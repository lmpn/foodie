use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait LoginService {
    async fn login(
        &self,
        email: String,
        password: String,
    ) -> Result<(String, i64), LoginServiceError>;
}

#[derive(Debug, PartialEq)]
pub enum LoginServiceError {
    InvalidCredentials,
    InternalError,
    PasswordHash,
    TokenEncodingError,
}

impl Display for LoginServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginServiceError::InternalError => f.write_str("Internal error"),
            LoginServiceError::InvalidCredentials => f.write_str("Invalid credentials"),
            LoginServiceError::PasswordHash => f.write_str("Password hashing error"),
            LoginServiceError::TokenEncodingError => f.write_str("Token encoding error"),
        }
    }
}
impl Error for LoginServiceError {}
