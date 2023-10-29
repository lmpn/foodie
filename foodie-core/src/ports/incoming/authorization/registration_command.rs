use crate::domain::authorization::filtered_user::FilteredUser;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

pub struct Request {
    name: String,
    email: String,
    password: String,
}

impl Request {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            name,
            email,
            password,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }
}

#[async_trait]
pub trait RegistrationCommand {
    async fn register(&self, request: Request) -> Result<FilteredUser, RegistrationCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum RegistrationCommandError {
    UserAlreadyExists,
    InternalError,
    PasswordHash,
}

impl Display for RegistrationCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistrationCommandError::InternalError => f.write_str("Internal error"),
            RegistrationCommandError::UserAlreadyExists => f.write_str("User already exists"),
            RegistrationCommandError::PasswordHash => f.write_str("Password hashing error"),
        }
    }
}
impl Error for RegistrationCommandError {}
