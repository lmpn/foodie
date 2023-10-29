use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait InsertRecipePort {
    async fn insert_recipe(
        &self,
        uuid: &str,
        name: &str,
        image: &str,
        method: &str,
    ) -> Result<(), InsertRecipeError>;
}

#[derive(Debug)]
pub enum InsertRecipeError {
    InternalError,
    RecipeAlreadyExists,
}

impl Display for InsertRecipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError => write!(f, "Internal error"),
            Self::RecipeAlreadyExists => write!(f, "Recipe already exists"),
        }
    }
}

impl Error for InsertRecipeError {}
