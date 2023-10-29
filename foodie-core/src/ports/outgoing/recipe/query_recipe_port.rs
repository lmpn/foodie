use crate::domain::recipe::recipe::Recipe;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait QueryRecipePort {
    async fn query_recipe(&self, uuid: uuid::Uuid) -> Result<Recipe, QueryRecipeError>;
}

#[derive(Debug)]
pub enum QueryRecipeError {
    RecordNotFound,
    InternalError,
}

impl Display for QueryRecipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RecordNotFound => write!(f, "Record not found"),
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for QueryRecipeError {}
