use async_trait::async_trait;
use std::{error::Error, fmt::Display};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Request {
    uuid: uuid::Uuid,
    name: String,
    image: String,
    method: String,
}

impl Request {
    pub fn new(uuid: uuid::Uuid, name: String, image: String, method: String) -> Self {
        Self {
            uuid,
            name,
            image,
            method,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn image(&self) -> &str {
        self.image.as_ref()
    }

    pub fn method(&self) -> &str {
        self.method.as_ref()
    }
}

#[async_trait]
pub trait UpdateRecipeCommand {
    async fn update_recipe(&self, request: Request) -> Result<(), UpdateRecipeCommandError>;
}

#[derive(Debug, PartialEq)]
pub enum UpdateRecipeCommandError {
    InternalError,
    RecipeNotFound,
}

impl Display for UpdateRecipeCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateRecipeCommandError::InternalError => f.write_str("internal error"),
            UpdateRecipeCommandError::RecipeNotFound => f.write_str("recipe not found"),
        }
    }
}
impl Error for UpdateRecipeCommandError {}
