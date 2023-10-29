use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait DeleteIngredientPort {
    async fn delete_ingredient(
        &self,
        recipe_uuid: &str,
        recipe_uuid: &str,
    ) -> Result<(), DeleteIngredientError>;
}

#[derive(Debug)]
pub enum DeleteIngredientError {
    InternalError,
}

impl Display for DeleteIngredientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for DeleteIngredientError {}
