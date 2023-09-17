use crate::application::domain::authorization::filtered_user::FilteredUser;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait RegistrationService {
    async fn register(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<FilteredUser, RegistrationServiceError>;
}

#[derive(Debug, PartialEq)]
pub enum RegistrationServiceError {
    UserAlreadyExists,
    InternalError,
    PasswordHash,
}

impl Display for RegistrationServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistrationServiceError::InternalError => f.write_str("Internal error"),
            RegistrationServiceError::UserAlreadyExists => f.write_str("User already exists"),
            RegistrationServiceError::PasswordHash => f.write_str("Password hashing error"),
        }
    }
}
impl Error for RegistrationServiceError {}
