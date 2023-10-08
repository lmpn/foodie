use async_trait::async_trait;
use std::{error::Error, fmt::Display};

use crate::application::domain::recipe::ingredient::Ingredient;

#[async_trait]
pub trait AddIngredientPort {
    async fn add_ingredient(
        &self,
        recipe_uuid: &str,
        ingredient: Ingredient,
    ) -> Result<(), AddIngredientError>;
}

#[derive(Debug)]
pub enum AddIngredientError {
    RecordNotFound,
    InternalError,
}

impl Display for AddIngredientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RecordNotFound => write!(f, "Record not found"),
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for AddIngredientError {}
