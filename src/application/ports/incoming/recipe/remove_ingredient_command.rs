use async_trait::async_trait;
use std::{error::Error, fmt::Display};
use uuid::Uuid;

#[async_trait]
pub trait RemoveCommand {
    async fn remove(&self, uuid: Uuid) -> Result<(), RemoveCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum RemoveCommandError {
    InternalError,
    RecipeNotFound,
    IngredientNotFound,
}

impl Display for RemoveCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoveCommandError::InternalError => f.write_str("Internal error"),
            RemoveCommandError::RecipeNotFound => f.write_str("Recipe not found"),
            RemoveCommandError::IngredientNotFound => f.write_str("Ingredient not found"),
        }
    }
}
impl Error for RemoveCommandError {}
