use async_trait::async_trait;

use std::{error::Error, fmt::Display};

use crate::services::domain::recipe::Recipe;
// #[automock(type Index = i64;)]
#[async_trait]
pub trait InsertRecipePort {
    async fn insert_recipe(&self, recipe: Recipe) -> Result<(), InsertRecipeError>;
}

#[derive(Debug)]
pub enum InsertRecipeError {
    InternalError,
}

impl Display for InsertRecipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for InsertRecipeError {}
