use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait UpdateIngredientPort {
    async fn update_ingredient(
        &self,
        uuid: &str,
        name: &str,
        amount: f64,
        unit: &str,
        recipe_uuid: &str,
    ) -> Result<(), UpdateIngredientError>;
}

#[derive(Debug)]
pub enum UpdateIngredientError {
    RecipeNotFound,
    InternalError,
}

impl Display for UpdateIngredientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RecipeNotFound => write!(f, "Recipe not found"),
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for UpdateIngredientError {}
