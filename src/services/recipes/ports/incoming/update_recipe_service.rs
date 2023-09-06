use std::{error::Error, fmt::Display};

use async_trait::async_trait;

#[async_trait]
pub trait UpdateRecipeService {
    async fn update_recipe(&self, buffer: Vec<u8>) -> Result<(), UpdateRecipeServiceError>;
}

#[derive(Debug, PartialEq)]
pub enum UpdateRecipeServiceError {
    InternalError,
}

impl Display for UpdateRecipeServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateRecipeServiceError::InternalError => f.write_str("Internal error"),
        }
    }
}
impl Error for UpdateRecipeServiceError {}
