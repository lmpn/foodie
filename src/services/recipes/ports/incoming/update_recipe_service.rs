use std::{error::Error, fmt::Display};

use async_trait::async_trait;

use crate::services::recipes::domain::recipe::Recipe;

#[async_trait]
pub trait UpdateRecipeService {
    async fn update_recipe(
        &self,
        recipe: Recipe,
        delete_ingredients: Vec<uuid::Uuid>,
    ) -> Result<(), UpdateRecipeServiceError>;
}

#[derive(Debug, PartialEq)]
pub enum UpdateRecipeServiceError {
    InternalError,
    RecipeNotFound,
}

impl Display for UpdateRecipeServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateRecipeServiceError::InternalError => f.write_str("Internal error"),
            UpdateRecipeServiceError::RecipeNotFound => f.write_str("Recipe not found"),
        }
    }
}
impl Error for UpdateRecipeServiceError {}
