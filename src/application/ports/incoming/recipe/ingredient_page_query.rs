use async_trait::async_trait;
use std::{error::Error, fmt::Display};
use uuid::Uuid;

use crate::application::domain::recipe::ingredient::Ingredient;

#[async_trait]
pub trait IngredientsPageQuery {
    async fn ingredients_page_query(
        &self,
        recipe_uuid: Uuid,
        count: i64,
        offset: i64,
    ) -> Result<Vec<Ingredient>, IngredientsPageQueryError>;
}

#[derive(Debug, PartialEq)]
pub enum IngredientsPageQueryError {
    RecipeNotFound,
    InternalError,
}

impl Display for IngredientsPageQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IngredientsPageQueryError::RecipeNotFound => f.write_str("Recipe not found"),
            IngredientsPageQueryError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for IngredientsPageQueryError {}
