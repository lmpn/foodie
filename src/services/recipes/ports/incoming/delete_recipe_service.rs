use std::{error::Error, fmt::Display};

use async_trait::async_trait;

#[async_trait]
pub trait DeleteRecipeService {
    async fn delete_recipe(&self, index: i64) -> Result<(), DeleteRecipeServiceError>;
}

#[derive(Debug, PartialEq)]
pub enum DeleteRecipeServiceError {
    RecipeNotFound,
    InternalError,
}

impl Display for DeleteRecipeServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteRecipeServiceError::RecipeNotFound => f.write_str("Recipe not found"),
            DeleteRecipeServiceError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for DeleteRecipeServiceError {}
