use crate::application::domain::recipe::ingredient::Ingredient;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait QueryRecipeIngredientPort {
    async fn query_recipe_ingredients(
        &self,
        recipe_uuid: uuid::Uuid,
        count: i64,
        offset: i64,
    ) -> Result<Vec<Ingredient>, QueryRecipeIngredientsError>;
}

#[derive(Debug)]
pub enum QueryRecipeIngredientsError {
    RecipeNotFound,
    InternalError,
}

impl Display for QueryRecipeIngredientsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RecipeNotFound => write!(f, "Recipe not found"),
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for QueryRecipeIngredientsError {}
