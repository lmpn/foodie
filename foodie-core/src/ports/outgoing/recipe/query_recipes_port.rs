use crate::domain::recipe::Recipe;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait QueryRecipesPort {
    async fn query_recipes(&self, count: u8, offset: u8) -> Result<Vec<Recipe>, QueryRecipesError>;
}

#[derive(Debug)]
pub enum QueryRecipesError {
    InternalError,
}

impl Display for QueryRecipesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for QueryRecipesError {}
