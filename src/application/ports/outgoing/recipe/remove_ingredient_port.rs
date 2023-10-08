use async_trait::async_trait;
use std::{error::Error, fmt::Display};

#[async_trait]
pub trait RemoveIngredientPort {
    async fn remove_ingredient(&self, uuid: &str) -> Result<(), RemoveIngredientError>;
}

#[derive(Debug)]
pub enum RemoveIngredientError {
    RecordNotFound,
    InternalError,
}

impl Display for RemoveIngredientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RecordNotFound => write!(f, "Record not found"),
            Self::InternalError => write!(f, "Internal error"),
        }
    }
}

impl Error for RemoveIngredientError {}
