use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait InsertRecipePort {
    async fn insert_recipe(
        &self,
        uuid: &str,
        name: &str,
        image: &str,
        method: &str,
    ) -> Result<(), InsertRecipeError>;
}

#[derive(Debug)]
pub enum InsertRecipeError {
    InternalError,
}

impl Display for InsertRecipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for InsertRecipeError {}
