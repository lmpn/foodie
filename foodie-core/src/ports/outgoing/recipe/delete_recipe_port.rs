use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait DeleteRecipePort {
    async fn delete_recipe(&self, uuid: &str) -> Result<(), DeleteRecipeError>;
}

#[derive(Debug)]
pub enum DeleteRecipeError {
    InternalError,
    RecipeNotFound,
}

impl Display for DeleteRecipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError => write!(f, "internal error"),
            Self::RecipeNotFound => write!(f, "recipe not found"),
        }
    }
}

impl Error for DeleteRecipeError {}
