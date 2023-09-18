use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait DeleteCommand {
    async fn delete(&self, uuid: uuid::Uuid) -> Result<(), DeleteCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum DeleteCommandError {
    RecipeNotFound,
    InternalError,
}

impl Display for DeleteCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteCommandError::RecipeNotFound => f.write_str("Recipe not found"),
            DeleteCommandError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for DeleteCommandError {}
