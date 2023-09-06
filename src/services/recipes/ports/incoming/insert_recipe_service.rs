use std::{error::Error, fmt::Display};

use async_trait::async_trait;

use crate::services::recipes::domain::recipe::Recipe;

#[async_trait]
pub trait InsertRecipeService {
    async fn insert_recipe(&self, recipe: Recipe) -> Result<(), InsertRecipeServiceError>;
}

#[derive(Debug, PartialEq)]
pub enum InsertRecipeServiceError {
    InternalError,
}

impl Display for InsertRecipeServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertRecipeServiceError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for InsertRecipeServiceError {}
