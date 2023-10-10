use crate::application::domain::recipe::recipe::Recipe;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait RecipeQuery {
    async fn recipe_query(&self, uuid: uuid::Uuid) -> Result<Recipe, RecipeQueryError>;
}

#[derive(Debug, PartialEq)]
pub enum RecipeQueryError {
    RecipeNotFound,
    InternalError,
}

impl Display for RecipeQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeQueryError::RecipeNotFound => f.write_str("Recipe not found"),
            RecipeQueryError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for RecipeQueryError {}
