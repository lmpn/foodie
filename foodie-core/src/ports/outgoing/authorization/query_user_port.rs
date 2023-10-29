use crate::domain::authorization::user::User;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait QueryUserPort {
    async fn query_user(&self, uuid: uuid::Uuid) -> Result<User, QueryUserError>;
}

#[derive(Debug, PartialEq)]
pub enum QueryUserError {
    UserNotFound,
    InternalError,
}

impl Display for QueryUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryUserError::UserNotFound => f.write_str("User not found"),
            QueryUserError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for QueryUserError {}
