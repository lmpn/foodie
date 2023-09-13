use std::{error::Error, fmt::Display};

use async_trait::async_trait;

use crate::services::domain::recipe::Recipe;

#[async_trait]
pub trait QueryRecipeService {
    async fn query_recipe(&self, uuid: uuid::Uuid) -> Result<Recipe, QueryRecipeServiceError>;
}

#[derive(Debug, PartialEq)]
pub enum QueryRecipeServiceError {
    RecipeNotFound,
    InternalError,
}

impl Display for QueryRecipeServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryRecipeServiceError::RecipeNotFound => f.write_str("Recipe not found"),
            QueryRecipeServiceError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for QueryRecipeServiceError {}
