use crate::application::domain::recipe::recipe::Recipe;
use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait ReadPartialQuery {
    async fn read_recipe(&self, uuid: uuid::Uuid) -> Result<Recipe, ReadPartialError>;
}

#[derive(Debug, PartialEq)]
pub enum ReadPartialError {
    RecipeNotFound,
    InternalError,
}

impl Display for ReadPartialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadPartialError::RecipeNotFound => f.write_str("Recipe not found"),
            ReadPartialError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for ReadPartialError {}
