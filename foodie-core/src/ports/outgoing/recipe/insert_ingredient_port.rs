use async_trait::async_trait;
use std::{error::Error, fmt::Display};

use crate::domain::recipe::ingredient::Ingredient;

#[async_trait]
pub trait InsertIngredientPort {
    async fn insert_ingredient(
        &self,
        recipe_uuid: &str,
        ingredient: Ingredient,
    ) -> Result<(), InsertIngredientError>;
}

#[derive(Debug)]
pub enum InsertIngredientError {
    RecipeNotFound,
    IngredientAlreadyExists,
    InternalError,
}

impl Display for InsertIngredientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RecipeNotFound => write!(f, "Recipe not found"),
            Self::IngredientAlreadyExists => write!(f, "Ingredient already exists"),
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for InsertIngredientError {}
