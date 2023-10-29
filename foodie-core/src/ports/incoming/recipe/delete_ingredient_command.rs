use async_trait::async_trait;
use std::{error::Error, fmt::Display};
use uuid::Uuid;

#[async_trait]
pub trait DeleteIngredientCommand {
    async fn delete(
        &self,
        recipe_uuid: Uuid,
        ingredient_uuid: Uuid,
    ) -> Result<(), DeleteIngredientCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum DeleteIngredientCommandError {
    InternalError,
}

impl Display for DeleteIngredientCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteIngredientCommandError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for DeleteIngredientCommandError {}
