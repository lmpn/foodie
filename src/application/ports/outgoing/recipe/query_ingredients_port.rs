use crate::application::domain::recipe::ingredient::Ingredient;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait QueryIngredientsPort {
    async fn query_ingredients(
        &self,
        recipe_uuid: &str,
        count: i64,
        offset: i64,
    ) -> Result<Vec<Ingredient>, QueryIngredientsError>;
}

#[derive(Debug)]
pub enum QueryIngredientsError {
    RecipeNotFound,
    InternalError,
}

impl Display for QueryIngredientsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RecipeNotFound => write!(f, "Recipe not found"),
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for QueryIngredientsError {}
