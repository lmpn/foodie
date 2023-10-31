use async_trait::async_trait;
use std::{error::Error, fmt::Display};

use crate::domain::authorization::token_claims::TokenClaims;

pub struct Request {
    email: String,
    password: String,
}

impl Request {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }
}

#[async_trait]
pub trait LoginCommand {
    async fn login(&self, request: Request) -> Result<TokenClaims, LoginCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum LoginCommandError {
    InvalidCredentials,
    InternalError,
    PasswordHash,
    TokenEncodingError,
}

impl Display for LoginCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginCommandError::InternalError => f.write_str("Internal error"),
            LoginCommandError::InvalidCredentials => f.write_str("Invalid credentials"),
            LoginCommandError::PasswordHash => f.write_str("Password hashing error"),
            LoginCommandError::TokenEncodingError => f.write_str("Token encoding error"),
        }
    }
}
impl Error for LoginCommandError {}
