use crate::services::recipes::domain::recipe::Recipe;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};
#[async_trait]
pub trait UpdateRecipePort {
    async fn update_recipe(&self, recipe: Recipe) -> Result<Recipe, UpdateRecipeError>;
}

#[derive(Debug)]
pub enum UpdateRecipeError {
    RecordNotFound,
    InternalError,
}

impl Display for UpdateRecipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RecordNotFound => write!(f, "Record not found"),
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for UpdateRecipeError {}
