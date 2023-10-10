use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait DeleteRecipeCommand {
    async fn delete_recipe(&self, uuid: uuid::Uuid) -> Result<(), DeleteRecipeCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum DeleteRecipeCommandError {
    RecipeNotFound,
    InternalError,
}

impl Display for DeleteRecipeCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteRecipeCommandError::RecipeNotFound => f.write_str("Recipe not found"),
            DeleteRecipeCommandError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for DeleteRecipeCommandError {}
