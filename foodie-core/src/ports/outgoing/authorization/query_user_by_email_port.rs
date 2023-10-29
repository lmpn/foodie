use crate::domain::authorization::user::User;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait QueryUserByEmailPort {
    async fn query_user_by_email(&self, email: &str) -> Result<User, QueryUserByEmailError>;
}

#[derive(Debug, PartialEq)]
pub enum QueryUserByEmailError {
    UserNotFound,
    InternalError,
}

impl Display for QueryUserByEmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryUserByEmailError::UserNotFound => f.write_str("User not found"),
            QueryUserByEmailError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for QueryUserByEmailError {}
