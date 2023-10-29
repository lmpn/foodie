use crate::domain::authorization::user::User;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

// #[automock(type Index = i64;)]
#[async_trait]
pub trait InsertUserPort {
    async fn insert_user(&self, user: User) -> Result<(), InsertUserError>;
}

#[derive(Debug)]
pub enum InsertUserError {
    InternalError,
    UserAlreadyExists,
}

impl Display for InsertUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertUserError::InternalError => write!(f, "Internal error"),
            InsertUserError::UserAlreadyExists => write!(f, "User already exists"),
        }
    }
}

impl Error for InsertUserError {}
