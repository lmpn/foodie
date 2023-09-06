use std::{error::Error, fmt::Display};

use async_trait::async_trait;
#[async_trait]
pub trait DeleteRecipePort {
    async fn delete_recipe(&self, index: i64) -> Result<String, DeleteRecipeError>;
}

#[derive(Debug)]
pub enum DeleteRecipeError {
    RecordNotFound,
    InternalError,
}

impl Display for DeleteRecipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RecordNotFound => write!(f, "Record not found"),
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for DeleteRecipeError {}
