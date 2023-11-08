use crate::domain::recipe::Recipe;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait RecipesPageQuery {
    async fn recipes_page_query(
        &self,
        count: u8,
        offset: u8,
    ) -> Result<Vec<Recipe>, RecipesPageQueryError>;
}

#[derive(Debug, PartialEq)]
pub enum RecipesPageQueryError {
    RecipeNotFound,
    InternalError,
    PageTooBig,
}

impl Display for RecipesPageQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipesPageQueryError::RecipeNotFound => f.write_str("Recipe not found"),
            RecipesPageQueryError::InternalError => f.write_str("Internal error"),
            RecipesPageQueryError::PageTooBig => {
                f.write_str("Number of recipes requested is too large")
            }
        }
    }
}
impl Error for RecipesPageQueryError {}
